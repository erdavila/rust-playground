use core::ptr::NonNull;

use intrusive_collections::Bound;
use intrusive_collections::rbtree::CursorMut;

use crate::interval::{Interval, LeftEndpoint, RightEndpoint};
use crate::{IntervalMap, Node, NodeAdapter, RawEntry, ValuesBetweenExist};

pub struct Insert<'a, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    state: Option<State<'a, K, V>>,
}
impl<'a, K, V> Insert<'a, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    pub(crate) fn new(map: &'a mut IntervalMap<K, V>, interval: Interval<K>, value: V) -> Self {
        Insert {
            state: Some(State::Initial(InitialState {
                map,
                inserting: RawEntry { interval, value },
            })),
        }
    }

    fn next_item<Out: OutputBuilder<K, V>>(&mut self) -> Option<Out::Output> {
        self.state.take().and_then(|mut state| {
            loop {
                match state.process::<Out>() {
                    (Some(output), Some(next_state)) => {
                        self.state = Some(next_state);
                        break Some(output);
                    }
                    (None, Some(next_state)) => state = next_state,
                    (output, None) => break output,
                }
            }
        })
    }
}
impl<K, V> Iterator for Insert<'_, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item::<IntervalAndValueOutput>()
    }
}
impl<K, V> Drop for Insert<'_, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    fn drop(&mut self) {
        while self.next_item::<UnitOutput>().is_some() {}
    }
}

trait Process<'a, K, V> {
    fn process<Out: OutputBuilder<K, V>>(self) -> (Option<Out::Output>, Option<State<'a, K, V>>);
}

enum State<'a, K, V> {
    Initial(InitialState<'a, K, V>),
    OtherNodes(OtherNodesState<'a, K, V>),
}
impl<'a, K, V> Process<'a, K, V> for State<'a, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    fn process<Out: OutputBuilder<K, V>>(self) -> (Option<Out::Output>, Option<State<'a, K, V>>) {
        match self {
            State::Initial(initial) => initial.process::<Out>(),
            State::OtherNodes(other_nodes) => other_nodes.process::<Out>(),
        }
    }
}

struct InitialState<'a, K, V> {
    map: &'a mut IntervalMap<K, V>,
    inserting: RawEntry<K, V>,
}
impl<'a, K, V> Process<'a, K, V> for InitialState<'a, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    fn process<Out: OutputBuilder<K, V>>(self) -> (Option<Out::Output>, Option<State<'a, K, V>>) {
        let InitialState { map, inserting } = self;

        if inserting.interval.is_empty() {
            return (None, None);
        }
        /*
            inserting:  {IL, IR} -> IV
            There MAY be nodes N where N.left <= IL and N.right is in [IL, IR].
            There MAY be nodes N where N.left is in (IL, IR].
        */

        let bound = Bound::Included(&inserting.left().as_ref());
        match Cursor::new(map.tree.upper_bound_mut(bound)) {
            Cursor::NonEmpty(non_empty) => {
                /*
                    inserting:     {IL, ⋯IR} -> IV
                    cursor:    {NL,     ⋯NR} -> NV
                    Where NL <= IL; NR <?> IL; NR <?> IR; NV =?= IV.
                    There are NO other nodes N where N.left is in (NL, IL].
                    There MAY be other nodes N where N.left is in (IL, IR].
                */
                Self::process_1::<Out>(inserting, non_empty)
            }
            Cursor::Empty(empty) => {
                /*
                    inserting: {IL, IR} -> IV
                    cursor:    -
                    There are NO node N where N.left <= IL and N.right is in [IL, IR].
                    There MAY be nodes N where N.left is in (IL, IR].
                */

                // Check if the first node intersects with the interval being inserted.
                match empty.first() {
                    Cursor::NonEmpty(non_empty) => {
                        /*
                            inserting: {IL,     ⋯IR} -> IV
                            cursor:        {NL, ⋯NR} -> NV
                            Where IL < NL; NL <?> IR; NR <?> IR; NV =?= IV.
                            There MAY be other nodes N where N.left is in (NR, IR].
                        */
                        Self::process_2::<Out>(inserting, non_empty)
                    }
                    Cursor::Empty(mut empty) => {
                        // No elements in the tree.
                        /*
                            inserting: {IL, IR} -> IV
                            cursor:    -
                        */
                        empty.insert(inserting);
                        (None, None)
                    }
                }
            }
        }
    }
}
impl<'a, K, V> InitialState<'a, K, V>
where
    K: ValuesBetweenExist + Clone + 'static,
    V: PartialEq + Clone,
{
    #[expect(clippy::too_many_lines)]
    fn process_1<Out: OutputBuilder<K, V>>(
        inserting: RawEntry<K, V>,
        mut cursor: NonEmptyCursor<'a, K, V>,
    ) -> (Option<Out::Output>, Option<State<'a, K, V>>) {
        let node = cursor.node();

        debug_assert!(node.left() <= inserting.left());
        /*
            inserting:       {IL, ⋯IR} -> IV
            cursor/node: {NL,     ⋯NR} -> NV
            Where NL <= IL; NR <?> IL; NR <?> IR; NV =?= IV.
            There are NO other nodes N where N.left is in (NL, IL].
            There MAY be other nodes N where N.left is in (IL, IR].
        */

        let other_nodes_state = Self::other_nodes_state_fn(&inserting, node);

        if node.right() < inserting.left() {
            // NR < IL
            /*
                inserting:           {IL, IR} -> IV
                cursor/node: {NL, NR}         -> NV
                Where NL <= NR < IL NR <= IR; NV =?= IV.
                There MAY be other nodes N where N.left is in (IL, IR].
            */

            // gap = @NR, IL@
            if node.value == inserting.value
                && let EmptinessBothExcluded::Empty(_gap) =
                    EmptinessBothExcluded::of(node.right(), inserting.left())
            {
                // NV = IV and `gap` is empty
                /*
                    inserting:           {IL, IR} -> V
                    cursor/node: {NL, NR}         -> V
                    gap:             @NR, IL@
                    Where NL <= NR < IL <= IR; `gap` is empty.
                    There MAY be other nodes N where N.left is in (IL, IR].
                */
                node.interval.right = inserting.interval.right;
                /*
                    cursor/node: {NL, IR} -> V
                */
                let inserted_ptr = cursor.node_ptr();
                let cursor = cursor.move_next();
                let output = None;
                /*
                    inserted: {__, IR} -> V
                    cursor:   ?
                    output:   -
                */

                let state = other_nodes_state(inserted_ptr, cursor);
                (output, Some(state))
            } else {
                // NV != IV or `gap` is NOT empty
                /*
                    inserting:           {IL, IR} -> IV
                    cursor/node: {NL, NR}         -> NV
                    gap:             @NR, IL@
                    Where NL <= NR < IL NR <= IR; `gap` is NOT empty; NV != IV.
                    There MAY be other nodes N where N.left is in (IL, IR].
                */
                cursor.insert_after_and_move_next(inserting);
                /*
                    cursor-1/node: {NL, NR}         -> NV
                    cursor:                {IL, IR} -> IV
                */
                let inserted_ptr = cursor.node_ptr();
                /*
                    cursor-1/node:   {NL, NR}         -> NV
                    cursor/inserted:         {IL, IR} -> IV
                */
                let cursor = cursor.move_next();
                /*
                    cursor-2/node:     {NL, NR}         -> NV
                    cursor-1/inserted:         {IL, IR} -> IV
                    cursor:            ?
                */
                let output = None;
                /*
                    cursor-2/node:     {NL, NR}         -> NV
                    cursor-1/inserted:         {IL, IR} -> IV
                    cursor:            ?
                    output:            -
                */

                let state = other_nodes_state(inserted_ptr, cursor);
                (output, Some(state))
            }
        } else {
            // NR >= IL
            /*
                inserting:       {IL, ⋯IR} -> IV
                cursor/node: {NL,     NR}  -> NV
                Where NL <= IL <= NR; NR <?> IR; NV =?= IV.
                There MAY be other nodes N where N.left is in (NR, IR].
            */

            // NR <?> IR
            let rights_cmp = node.right().cmp(inserting.right());
            if rights_cmp.is_le() {
                // NR <= IR
                /*
                    inserting:       {IL,     IR} -> IV
                    cursor/node: {NL,     NR}     -> NV
                    Where NL <= IL <= NR <= IR; NV =?= IV.
                    There MAY be other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserting.value {
                    // NV = IV
                    /*
                        inserting:       {IL,     IR} -> V
                        cursor/node: {NL,     NR}     -> V
                        Where NL <= IL <= NR <= IR.
                        There MAY be other nodes N where N.left is in (NR, IR].
                    */
                    let out_right = node.replace_right(inserting.interval.right);
                    /*
                        inserting:       {IL,     __} -> V
                        cursor/node: {NL,         IR} -> V
                        output:          {__, NR}     -> _
                    */
                    let output = Out::from_fn(|| RawEntry {
                        interval: Interval {
                            left: inserting.interval.left,
                            right: out_right,
                        },
                        value: inserting.value,
                    });
                    /*
                        cursor/node: {NL,         IR} -> V
                        output:          {IL, NR}     -> V
                    */

                    if rights_cmp.is_lt() {
                        // NR < IR
                        /*
                            cursor/node: {NL,         IR} -> V
                            output:          {IL, NR}     -> V
                            Where NL <= IL <= NR < IR.
                            There MAY be other nodes N where N.left is in (NR, IR].
                        */
                        let inserted_ptr = cursor.node_ptr();
                        let cursor = cursor.move_next();
                        /*
                            inserted: {__,         IR} -> V
                            cursor:   ?
                            output:       {IL, NR}     -> V
                        */

                        let state = other_nodes_state(inserted_ptr, cursor);
                        (Some(output), Some(state))
                    } else {
                        // NR = IR
                        /*
                            cursor/node: {NL,     IR} -> V
                            output:          {IL, NR}     -> V
                            Where NL <= IL <= NR = IR.
                            There are NO other nodes N where N.left is in (NR, IR].
                        */
                        (Some(output), None)
                    }
                } else {
                    // NV != IV
                    /*
                        inserting:       {IL,     IR} -> IV
                        cursor/node: {NL,     NR}     -> NV
                        Where NL <= IL <= NR <= IR; NV != IV.
                        There MAY be other nodes N where N.left is in (NR, IR].
                    */

                    // before = {NL, IL@
                    let output = match EmptinessRightExcluded::of(node.left(), inserting.left()) {
                        EmptinessRightExcluded::Empty(_before) => {
                            // {NL, IL@ is empty
                            /*
                                inserting:       {IL,     IR} -> IV
                                cursor/node: {NL,     NR}     -> NV
                                before:      {NL, IL@
                                Where NL <= IL <= NR <= IR; `before` is empty; NV != IV.
                                There MAY be other nodes N where N.left is in (NR, IR].
                            */
                            let output = Out::from(node.replace(inserting));
                            /*
                                cursor/node:     {IL,     IR} -> IV
                                output:      {NL,     NR}     -> NV
                            */
                            output
                        }
                        EmptinessRightExcluded::NonEmpty(before) => {
                            // {NL, IL@ is NOT empty
                            /*
                                inserting:       {IL,     IR} -> IV
                                cursor/node: {NL,     NR}     -> NV
                                before:      {NL, IL@
                                Where NL <= IL <= NR <= IR; `before` is NOT empty; NV != IV.
                                There MAY be other nodes N where N.left is in (NR, IR].
                            */
                            let out_right = node.replace_right(before.right.cloned());
                            /*
                                inserting:       {IL,     IR} -> IV
                                cursor/node: {NL, IL@         -> NV
                                output:          {__, NR}     -> __
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: inserting.left().clone(),
                                    right: out_right,
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserting:       {IL,     IR} -> IV
                                cursor/node: {NL, IL@         -> NV
                                output:          {IL, NR}     -> NV
                            */
                            cursor.insert_after_and_move_next(inserting);
                            /*
                                cursor-1: {NL, IL@         -> NV
                                cursor:       {IL,     IR} -> IV
                                output:       {IL, NR}     -> NV
                            */
                            output
                        }
                        EmptinessRightExcluded::RightInfinity(_left) => {
                            // IL = -inf
                            debug_assert!(node.left() == LeftEndpoint::Infinity); // NL = -inf too.
                            //
                            /*
                                inserting:   (-inf,     IR} -> IV
                                cursor/node: (-inf, NR}     -> NV
                                left:        (-inf
                                Where -inf <= NR <= IR; NV != IV.
                                There MAY be other nodes N where N.left is in (NR, IR].
                            */
                            let output = Out::from(node.replace(inserting));
                            /*
                                cursor/node: (-inf,     IR} -> IV
                                output:      (-inf, NR}     -> NV
                            */
                            output
                        }
                    };

                    if rights_cmp.is_lt() {
                        // NR < IR
                        /*
                            cursor: {__,     IR} -> IV
                            output: {__, NR}     -> NV
                        */
                        let inserted_ptr = cursor.node_ptr();
                        let cursor = cursor.move_next();
                        /*
                            inserted: {__,     IR} -> IV
                            cursor:   ?
                            output:   {__, NR}     -> NV
                        */

                        let state = other_nodes_state(inserted_ptr, cursor);
                        (Some(output), Some(state))
                    } else {
                        // NR = IR
                        (Some(output), None)
                    }
                }
            } else {
                // NR > IR
                /*
                    inserting:       {IL, IR}     -> IV
                    cursor/node: {NL,         NR} -> NV
                    Where NL <= IL <= IR < NR; NV =?= IV.
                    There are NO other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserting.value {
                    // NV = IV
                    /*
                        inserting:       {IL, IR}     -> V
                        cursor/node: {NL,         NR} -> V
                        Where NL <= IL <= IR < NR.
                    */
                    let output = Out::from(inserting);
                    /*
                        cursor/node: {NL,         NR} -> V
                        output:          {IL, IR}     -> V
                    */
                    (Some(output), None)
                } else {
                    // NV != IV
                    /*
                        inserting:       {IL, IR}     -> IV
                        cursor/node: {NL,         NR} -> NV
                        Where NL <= IL <= IR < NR; NV != IV.
                    */

                    // before = {NL, IL@
                    let before_emptiness =
                        EmptinessRightExcluded::of(node.left(), inserting.left());

                    // after = @IR, NR}
                    let after_emptiness =
                        EmptinessLeftExcluded::of(inserting.right(), node.right());

                    match (before_emptiness, after_emptiness) {
                        (
                            EmptinessRightExcluded::Empty(_before),
                            EmptinessLeftExcluded::Empty(_after),
                        ) => {
                            // `before` is empty, `after` is empty
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL,         NR} -> NV
                                before:      {NL, IL@
                                after:               @IR, NR}
                                Where NL <= IL <= IR < NR; before` is empty; `after` is empty; NV != IV.
                            */
                            let output = Out::from(node.replace(inserting));
                            /*
                                cursor/node:     {IL, IR}     -> IV
                                output:      {NL,         NR} -> NV
                            */
                            (Some(output), None)
                        }
                        (
                            EmptinessRightExcluded::Empty(_before),
                            EmptinessLeftExcluded::NonEmpty(after),
                        ) => {
                            // `before` is empty, `after` is NOT empty
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL,         NR} -> NV
                                before:      {NL, IL@
                                after:               @IR, NR}
                                Where NL <= IL <= IR < NR; before` is empty; `after` is NOT empty; NV != IV.
                            */
                            let out_left = node.replace_left(after.left.cloned());
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:      {NL,     __}     -> __
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: out_left,
                                    right: inserting.right().clone(),
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:      {NL,     IR}     -> NV
                            */
                            cursor.insert_before(inserting);
                            /*
                                cursor-1:        {IL, IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:      {NL,     IR}     -> NV
                            */
                            (Some(output), None)
                        }
                        (
                            EmptinessRightExcluded::NonEmpty(before),
                            EmptinessLeftExcluded::Empty(_after),
                        ) => {
                            // `before` is NOT empty, `after` is empty
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL,         NR} -> NV
                                before:      {NL, IL@
                                after:               @IR, NR}
                                Where NL <= IL <= IR < NR; before` is NOT empty; `after` is empty; NV != IV.
                            */
                            let out_right = node.replace_right(before.right.cloned());
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL, IL@         -> NV
                                output:          {__,     NR} -> __
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: inserting.left().clone(),
                                    right: out_right,
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL, IL@         -> NV
                                output:          {IL,     NR} -> NV
                            */
                            cursor.insert_after(inserting);
                            /*
                                cursor/node: {NL, IL@         -> NV
                                cursor+1:        {IL, IR}     -> IV
                                output:          {IL,     NR} -> NV
                            */
                            (Some(output), None)
                        }
                        (
                            EmptinessRightExcluded::NonEmpty(before),
                            EmptinessLeftExcluded::NonEmpty(after),
                        ) => {
                            // `before` is NOT empty, `after` is NOT empty
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL,         NR} -> NV
                                before:      {NL, IL@
                                after:               @IR, NR}
                                Where NL <= IL <= IR < NR; before` is NOT empty; `after` is NOT empty; NV != IV.
                            */
                            let after_left = after.left.cloned();
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL,         NR} -> NV
                                after:               @IR, __} -> __
                            */
                            let after_right = node.replace_right(before.right.cloned());
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL, IL@         -> NV
                                after:               @IR, NR} -> __
                            */
                            let after = RawEntry {
                                interval: Interval {
                                    left: after_left,
                                    right: after_right,
                                },
                                value: node.value.clone(),
                            };
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL, IL@         -> NV
                                after:               @IR, NR} -> NV
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: inserting.interval.clone(),
                                value: node.value.clone(),
                            });
                            /*
                                inserting:       {IL, IR}     -> IV
                                cursor/node: {NL, IL@         -> NV
                                after:               @IR, NR} -> NV
                                output:          {IL, IR}     -> NV
                            */
                            cursor.insert_after(after);
                            /*
                                inserting:     {IL, IR}     -> IV
                                cursor:    {NL, IL@         -> NV
                                cursor+1:          @IR, NR} -> NV
                                output:        {IL, IR}     -> NV
                            */
                            cursor.insert_after(inserting);
                            /*
                                cursor:    {NL, IL@         -> NV
                                cursor+1:      {IL, IR}     -> IV
                                cursor+2:          @IR, NR} -> NV
                                output:        {IL, IR}     -> NV
                            */
                            (Some(output), None)
                        }
                        (
                            EmptinessRightExcluded::RightInfinity(_left),
                            EmptinessLeftExcluded::Empty(_after),
                        ) => {
                            // IL = -inf, `after` is empty
                            debug_assert!(node.left() == LeftEndpoint::Infinity); // NL is -inf too.
                            //
                            /*
                                inserting:   (-inf, IR}     -> IV
                                cursor/node: (-inf,     NR} -> NV
                                after:             @IR, NR}
                                Where -inf <= IR < NR; `after` is empty; NV != IV.
                            */
                            let output = Out::from(node.replace(inserting));
                            /*
                                cursor/node: (-inf, IR}     -> IV
                                output:      (-inf,     NR} -> NV
                            */
                            (Some(output), None)
                        }
                        (
                            EmptinessRightExcluded::RightInfinity(_left),
                            EmptinessLeftExcluded::NonEmpty(after),
                        ) => {
                            // IL = -inf, `after` is NOT empty
                            debug_assert!(node.left() == LeftEndpoint::Infinity); // NL is -inf too.
                            //
                            /*
                                inserting:   (-inf, IR}     -> IV
                                cursor/node: (-inf,     NR} -> NV
                                after:             @IR, NR}
                                Where -inf <= IR < NR; `after` is NOT empty; NV != IV.
                            */
                            node.interval.left = after.left.cloned();
                            /*
                                inserting:   (-inf, IR}     -> IV
                                cursor/node:       @IR, NR} -> NV
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: LeftEndpoint::Infinity,
                                    right: inserting.right().clone(),
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserting:   (-inf, IR}     -> IV
                                cursor/node:       @IR, NR} -> NV
                                output:      (-inf, IR}     -> NV
                            */
                            cursor.insert_before(inserting);
                            /*
                                cursor-1:    (-inf, IR}     -> IV
                                cursor/node:       @IR, NR} -> NV
                                output:      (-inf, IR}     -> NV
                            */
                            (Some(output), None)
                        }
                        (_, EmptinessLeftExcluded::LeftInfinity(_right)) => {
                            // IR = +inf
                            /*
                                inserting:       {IL, +inf)   -> IV
                                cursor/node: {NL,         NR} -> NV
                                Where NL <= IL <= +inf < NR (!!!); NV != IV.
                            */
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    fn process_2<Out: OutputBuilder<K, V>>(
        inserting: RawEntry<K, V>,
        mut cursor: NonEmptyCursor<'a, K, V>,
    ) -> (Option<Out::Output>, Option<State<'a, K, V>>) {
        let node = cursor.node();
        debug_assert!(node.left() > inserting.left());

        /*
            inserting:   {IL,     ⋯IR} -> IV
            cursor/node:     {NL, ⋯NR} -> NV
            Where IL < NL; NL <?> IR; NR <?> IR; NV =?= IV.
            There MAY be other nodes N where N.left is in (NR, IR].
        */

        let other_nodes_state = Self::other_nodes_state_fn(&inserting, node);

        if node.left() > inserting.right() {
            // NL > IR
            /*
                inserting:   {IL, IR}         -> IV
                cursor/node:         {NL, NR} -> NV
                Where IL <= IR < NL <= NR; NV =?= IV.
                There are NO other nodes N where N.left is in (NR, IR].
            */

            // gap = @IR, NL@
            if node.value == inserting.value
                && let EmptinessBothExcluded::Empty(_gap) =
                    EmptinessBothExcluded::of(inserting.right(), node.left())
            {
                // NV = IV and `gap` is empty
                /*
                    inserting:   {IL, IR}         -> V
                    cursor/node:         {NL, NR} -> V
                    gap:             @IR, NL@
                    Where IL <= IR < NL <= NR; `gap` is empty.
                */
                node.interval.left = inserting.interval.left;
                let output = None;
                /*
                    cursor/node: {IL,         NR} -> V
                    output:      -
                */
                (output, None)
            } else {
                // NV != IV or `gap` is NOT empty
                /*
                    inserting:   {IL, IR}         -> IV
                    cursor/node:         {NL, NR} -> NV
                    Where IL <= IR < NL <= NR; NV =?= IV.
                */
                cursor.insert_before(inserting);
                let output = None;
                /*
                    cursor-1:    {IL, IR}         -> IV
                    cursor/node:         {NL, NR} -> NV
                    output:      -
                */
                (output, None)
            }
        } else {
            // NL <= IR
            /*
                inserting:   {IL,     IR}  -> IV
                cursor/node:     {NL, ⋯NR} -> NV
                Where IL < NL <= IR; NR <?> IR; NV =?= IV.
                There MAY be other nodes N where N.left is in (NR, IR].
            */

            // NR <?> IR
            let rights_cmp = node.right().cmp(inserting.right());
            if rights_cmp.is_le() {
                // NR <= IR
                /*
                    inserting:   {IL,         IR} -> IV
                    cursor/node:     {NL, NR}     -> NV
                    Where IL < NL <= NR <= IR; NV =?= IV.
                    There MAY be other nodes N where N.left is in (NR, IR].
                */

                let output = Out::from(node.replace(inserting));
                /*
                    cursor/node: {IL,         IR} -> IV
                    output:          {NL, NR}     -> NV
                */

                if rights_cmp.is_lt() {
                    // NR < IR
                    /*
                        cursor/node: {IL,         IR} -> IV
                        output:          {NL, NR}     -> NV
                        Where IL < NL <= NR < IR; NV =?= IV.
                        There MAY be other nodes N where N.left is in (NR, IR].
                    */
                    let inserted_ptr = cursor.node_ptr();
                    let cursor = cursor.move_next();
                    /*
                        inserted: {__,         IR} -> IV
                        cursor:   ?
                        output:       {NL, NR}     -> NV
                    */

                    let state = other_nodes_state(inserted_ptr, cursor);
                    (Some(output), Some(state))
                } else {
                    // NR = IR
                    /*
                        cursor/node: {IL,     IR} -> IV
                        output:          {NL, NR} -> NV
                        Where IL < NL <= NR = IR; NV =?= IV.
                        There are NO other nodes N where N.left is in (NR, IR].
                    */
                    (Some(output), None)
                }
            } else {
                // NR > IR
                /*
                    inserting:   {IL,     IR}     -> IV
                    cursor/node:     {NL,     NR} -> NV
                    Where IL < NL <= IR < NR; NV =?= IV.
                    There are NO other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserting.value {
                    // NV = IV
                    /*
                        inserting:   {IL,     IR}     -> V
                        cursor/node:     {NL,     NR} -> V
                        Where IL < NL <= IR < NR.
                    */
                    let out_left = node.replace_left(inserting.interval.left);
                    /*
                        inserting:   {__,     IR}     -> V
                        cursor/node: {IL,         NR} -> V
                        output:          {NL, __}     -> _
                    */
                    let output = Out::from(RawEntry {
                        interval: Interval {
                            left: out_left,
                            right: inserting.interval.right,
                        },
                        value: inserting.value,
                    });
                    /*
                        cursor/node: {IL,         NR} -> V
                        output:          {NL, IR}     -> V
                    */
                    (Some(output), None)
                } else {
                    // NV != IV
                    /*
                        inserting:   {IL,     IR}     -> IV
                        cursor/node:     {NL,     NR} -> NV
                        Where IL < NL <= IR < NR; NV != IV.
                    */

                    // after = @IR, NR}
                    match EmptinessLeftExcluded::of(inserting.right(), node.right()) {
                        EmptinessLeftExcluded::Empty(_after) => {
                            // @IR, NR} is empty
                            /*
                                inserting:   {IL,     IR}     -> IV
                                cursor/node:     {NL,     NR} -> NV
                                after:               @IR, NR}
                                Where IL < NL <= IR < NR; `after` is empty; NV != IV.
                            */
                            let output = Out::from(node.replace(inserting));
                            /*
                                cursor/node: {IL,     IR}     -> IV
                                output:          {NL,     NR} -> NV
                            */
                            (Some(output), None)
                        }
                        EmptinessLeftExcluded::NonEmpty(after) => {
                            // @IR, NR} is NOT empty
                            /*
                                inserting:   {IL,     IR}     -> IV
                                cursor/node:     {NL,     NR} -> NV
                                after:               @IR, NR}
                                Where IL < NL <= IR < NR; `after` is NOT empty; NV != IV.
                            */
                            let out_left = node.replace_left(after.left.cloned());
                            /*
                                inserting:   {IL,     IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:          {NL, __}     -> __
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: out_left,
                                    right: inserting.right().clone(),
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserting:   {IL,     IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:          {NL, IR}     -> NV
                            */
                            cursor.insert_before(inserting);
                            /*
                                cursor-1:    {IL,     IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:          {NL, IR}     -> NV
                            */
                            (Some(output), None)
                        }
                        EmptinessLeftExcluded::LeftInfinity(_right) => {
                            // IR = +inf
                            /*
                                inserting:   {IL,   +inf)     -> IV
                                cursor/node:     {NL,     NR} -> NV
                                Where IL < NL <= +inf < NR (!!!); NV != IV.
                            */
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    fn other_nodes_state_fn(
        inserting: &RawEntry<K, V>,
        node: &Node<K, V>,
    ) -> impl Fn(NonNull<Node<K, V>>, Cursor<'a, K, V>) -> State<'a, K, V> + use<'a, K, V> {
        #[cfg(debug_assertions)]
        let node_right = node.right().clone();
        #[cfg(debug_assertions)]
        let inserting_right = inserting.right().clone();
        #[cfg(debug_assertions)]
        let inserting_value = inserting.value.clone();

        move |inserted_ptr: NonNull<Node<_, _>>, mut cursor| {
            #[cfg(debug_assertions)]
            if let Cursor::NonEmpty(cursor) = &mut cursor {
                assert!(cursor.node().left() > node_right);
                let inserted = unsafe { inserted_ptr.as_ref() };
                assert!(inserted.right() == inserting_right);
                assert!(inserted.value == inserting_value);
            }

            State::OtherNodes(OtherNodesState {
                inserted_ptr,
                cursor,
            })
        }
    }
}

struct OtherNodesState<'a, K, V> {
    // {__, IR} -> IV
    inserted_ptr: NonNull<Node<K, V>>,
    cursor: Cursor<'a, K, V>,
}
impl<'a, K, V> Process<'a, K, V> for OtherNodesState<'a, K, V>
where
    K: ValuesBetweenExist + Clone,
    V: PartialEq + Clone,
{
    fn process<Out: OutputBuilder<K, V>>(self) -> (Option<Out::Output>, Option<State<'a, K, V>>) {
        let OtherNodesState {
            mut inserted_ptr,
            cursor,
        } = self;
        let inserted = unsafe { inserted_ptr.as_mut() };
        /*
            inserted: {__, IR} -> IV
            `inserted.left` is meaningless.
        */

        let Cursor::NonEmpty(mut cursor) = cursor else {
            // No more nodes.
            return (None, None);
        };

        let node = cursor.node();
        /*
            inserted:    {__, ⋯IR} -> IV
            cursor/node: {NL⋯, ⋯NR} -> NV
            Where NL <?> IR; NR <?> IR; NV =?= IV.
            There MAY be other nodes N where N.left is in (NR, IR].
        */

        #[cfg(debug_assertions)]
        let node_right = node.right().clone();

        let other_nodes_state = |mut cursor| {
            #[cfg(debug_assertions)]
            if let Cursor::NonEmpty(cursor) = &mut cursor {
                assert!(cursor.node().left() > node_right);
            }

            State::OtherNodes(OtherNodesState {
                inserted_ptr,
                cursor,
            })
        };

        // NL <?> IR
        if node.left() <= inserted.right() {
            // NL <= IR
            /*
                inserted:    {__,     IR}  -> IV
                cursor/node:     {NL, ⋯NR} -> NV
                Where NL <= IR; NR <?> IR; NV =?= IV.
                There MAY be other nodes N where N.left is in (NR, IR].
            */

            // NR <?> IR
            let rights_cmp = node.right().cmp(inserted.right());
            if rights_cmp.is_le() {
                // NR <= IR
                /*
                    inserted:    {__,         IR} -> IV
                    cursor/node:     {NL, NR}     -> NV
                    Where NL <= NR <= IR; NV =?= IV.
                    There MAY be other nodes N where N.left is in (NR, IR].
                */
                let (output, cursor) = cursor.remove_as_output::<Out>();
                /*
                    inserted: {__,         IR} -> IV
                    cursor:   ?
                    output:       {NL, NR}     -> NV
                */

                if rights_cmp.is_lt() {
                    // NR < IR
                    /*
                        inserted: {__,         ⋯IR} -> IV
                        cursor:               {NL', NR'} -> NV'
                        output:       {NL, NR}           -> NV
                        Where NL <= NR < IR; NR < NL' <= NR'; NV =?= IV.
                        There MAY be other nodes N where N.left is in (NR, IR].
                    */

                    let state = other_nodes_state(cursor);
                    (Some(output), Some(state))
                } else {
                    // NR = IR
                    /*
                        inserted: {__,     IR} -> IV
                        cursor:   ?
                        output:       {NL, NR} -> NV
                        Where NL <= NR = IR; NV =?= IV.
                        There are NO other nodes N where N.left is in (NR, IR].
                    */
                    (Some(output), None)
                }
            } else {
                // NR > IR
                /*
                    inserted:    {__,     IR}     -> IV
                    cursor/node:     {NL,     NR} -> NV
                    Where NL <= IR < IR; NV =?= IV.
                    There are NO other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserted.value {
                    // NV = IV
                    /*
                        inserted:    {__,     IR}     -> V
                        cursor/node:     {NL,     NR} -> V
                        Where NL <= IR < IR.
                    */
                    node.swap_right(inserted.right_mut());
                    /*
                        inserted:    {__,         NR} -> V
                        cursor/node:     {NL, IR}     -> V
                    */
                    let (output, _) = cursor.remove_as_output::<Out>();
                    /*
                        inserted: {__,         NR} -> V
                        output:       {NL, IR}     -> V
                    */
                    (Some(output), None)
                } else {
                    // NV != IV
                    /*
                        inserted:    {__,     IR}     -> IV
                        cursor/node:     {NL,     NR} -> NV
                        Where NL <= IR < IR; NV != IV.
                    */

                    // after = @IR, NR}
                    match EmptinessLeftExcluded::of(inserted.right(), node.right()) {
                        EmptinessLeftExcluded::Empty(_after) => {
                            // `after` is empty
                            /*
                                inserted:    {__,     IR}     -> IV
                                cursor/node:     {NL,     NR} -> NV
                                after:               @IR, NR}
                                Where NL <= IR < IR; `after` is empty; NV != IV.
                            */
                            let (output, _) = cursor.remove_as_output::<Out>();
                            /*
                                inserted:    {__,     IR}     -> IV
                                output:          {NL,     NR} -> NV
                            */
                            (Some(output), None)
                        }
                        EmptinessLeftExcluded::NonEmpty(after) => {
                            // `after` is NOT empty
                            /*
                                inserted:    {__,     IR}     -> IV
                                cursor/node:     {NL,     NR} -> NV
                                after:               @IR, NR}
                                Where NL <= IR < IR; `after` is NOT empty; NV != IV.
                            */
                            let out_left = node.replace_left(after.left.cloned());
                            /*
                                inserted:    {__,     IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:          {NL, __}     -> __
                            */
                            let output = Out::from_fn(|| RawEntry {
                                interval: Interval {
                                    left: out_left,
                                    right: inserted.right().clone(),
                                },
                                value: node.value.clone(),
                            });
                            /*
                                inserted:    {__,     IR}     -> IV
                                cursor/node:         @IR, NR} -> NV
                                output:          {NL, IR}     -> NV
                            */
                            (Some(output), None)
                        }
                        EmptinessLeftExcluded::LeftInfinity(_right) => {
                            // IR = +inf
                            /*
                                inserted:    {__,     +inf)     -> IV
                                cursor/node:     {NL,       NR} -> NV
                                Where NL <= +inf < IR (!!!); NV != IV.
                            */
                            unreachable!()
                        }
                    }
                }
            }
        } else {
            // NL > IR
            /*
                inserted:    {__, IR}         -> IV
                cursor/node:         {NL, NR} -> NV
                Where IR < NL <= NR; NV =?= IV.
                There are NO other nodes N where N.left is in (NR, IR].
            */

            // gap = @IR, NL@
            if node.value == inserted.value
                && let EmptinessBothExcluded::Empty(_gap) =
                    EmptinessBothExcluded::of(inserted.right(), node.left())
            {
                // NV = IV and `gap` is empty
                /*
                    inserted:    {__, IR}         -> V
                    cursor/node:         {NL, NR} -> V
                    Where IR < NL <= NR; `gap` is empty`; NV =?= IV.
                */
                // Merge the nodes.
                let (removed, _) = cursor.remove();
                /*
                    inserted: {__, IR}         -> V
                    removed:          {NL, NR} -> V
                    cursor:   ?
                */
                inserted.interval.right = removed.interval.right;
                /*
                    inserted: {__,         NR} -> V
                    removed:          {NL, __} -> V
                    cursor:   ?
                */
                let output = None;
                /*
                    inserted: {__,         NR} -> V
                    removed:          {NL, __} -> V
                    cursor:   ?
                    output:   -
                */
                (output, None)
            } else {
                // NV != IV or `gap` is NOT empty
                /*
                    inserted:    {__, IR}         -> IV
                    cursor/node:         {NL, NR} -> NV
                    Where IR < NL <= NR; NV =?= IV.
                */
                (None, None)
            }
        }
    }
}

trait OutputBuilder<K, V> {
    type Output;

    fn from(interval_and_value: impl Into<(Interval<K>, V)>) -> Self::Output;

    fn from_fn<T: Into<(Interval<K>, V)>>(f: impl FnOnce() -> T) -> Self::Output;
}

struct IntervalAndValueOutput;
impl<K, V> OutputBuilder<K, V> for IntervalAndValueOutput {
    type Output = (Interval<K>, V);

    fn from(interval_and_value: impl Into<(Interval<K>, V)>) -> Self::Output {
        interval_and_value.into()
    }

    fn from_fn<T: Into<(Interval<K>, V)>>(f: impl FnOnce() -> T) -> Self::Output {
        f().into()
    }
}

struct UnitOutput;
impl<K, V> OutputBuilder<K, V> for UnitOutput {
    type Output = ();

    fn from(_interval_and_value: impl Into<(Interval<K>, V)>) -> Self::Output {}

    fn from_fn<T: Into<(Interval<K>, V)>>(_f: impl FnOnce() -> T) -> Self::Output {}
}

enum EmptinessLeftExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    LeftInfinity(RightEndpoint<&'a K>),
}
impl<'a, K> EmptinessLeftExcluded<'a, K>
where
    K: ValuesBetweenExist,
{
    fn of(left: &'a RightEndpoint<K>, right: &'a RightEndpoint<K>) -> Self {
        let right = right.as_ref();
        if let Some(left) = left.opposite() {
            let interval = Interval { left, right };
            if interval.is_empty() {
                EmptinessLeftExcluded::Empty(interval)
            } else {
                EmptinessLeftExcluded::NonEmpty(interval)
            }
        } else {
            EmptinessLeftExcluded::LeftInfinity(right)
        }
    }
}

enum EmptinessRightExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    RightInfinity(LeftEndpoint<&'a K>),
}
impl<'a, K> EmptinessRightExcluded<'a, K> {
    fn of(left: &'a LeftEndpoint<K>, right: &'a LeftEndpoint<K>) -> Self
    where
        K: ValuesBetweenExist,
    {
        let left = left.as_ref();
        if let Some(right) = right.opposite() {
            let interval = Interval { left, right };
            if interval.is_empty() {
                EmptinessRightExcluded::Empty(interval)
            } else {
                EmptinessRightExcluded::NonEmpty(interval)
            }
        } else {
            EmptinessRightExcluded::RightInfinity(left)
        }
    }
}

#[expect(unused)]
enum EmptinessBothExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    LeftInfinity(RightEndpoint<&'a K>),
    RightInfinity(LeftEndpoint<&'a K>),
    BothInfinity,
}
impl<'a, K> EmptinessBothExcluded<'a, K> {
    fn of(left: &'a RightEndpoint<K>, right: &'a LeftEndpoint<K>) -> EmptinessBothExcluded<'a, K>
    where
        K: ValuesBetweenExist,
    {
        match (left.opposite(), right.opposite()) {
            (Some(left), Some(right)) => {
                let interval = Interval { left, right };
                if interval.is_empty() {
                    EmptinessBothExcluded::Empty(interval)
                } else {
                    EmptinessBothExcluded::NonEmpty(interval)
                }
            }
            (Some(left), None) => EmptinessBothExcluded::RightInfinity(left),
            (None, Some(right)) => EmptinessBothExcluded::LeftInfinity(right),
            (None, None) => EmptinessBothExcluded::BothInfinity,
        }
    }
}

enum Cursor<'a, K, V> {
    NonEmpty(NonEmptyCursor<'a, K, V>),
    Empty(EmptyCursor<'a, K, V>),
}
impl<'a, K, V> Cursor<'a, K, V> {
    fn new(inner: CursorMut<'a, NodeAdapter<K, V>>) -> Self {
        if inner.is_null() {
            Self::Empty(EmptyCursor { inner })
        } else {
            Self::NonEmpty(NonEmptyCursor { inner })
        }
    }
}

// Provides a safe API that (supposedly) avoid UB.
struct NonEmptyCursor<'a, K, V> {
    // Must not be null.
    inner: CursorMut<'a, NodeAdapter<K, V>>,
}
impl<'a, K, V> NonEmptyCursor<'a, K, V> {
    // Captures the cursor lifetime, so it cannot be used while the reference is alive.
    fn node(&mut self) -> &mut Node<K, V> {
        unsafe { self.node_ptr().as_mut() }
    }

    fn node_ptr(&self) -> NonNull<Node<K, V>> {
        self.inner.get_ptr().expect("null cursor")
    }

    #[must_use]
    fn move_next(mut self) -> Cursor<'a, K, V> {
        self.inner.move_next();
        Cursor::new(self.inner)
    }

    fn insert_after(&mut self, entry: impl Into<(Interval<K>, V)>) {
        let node = Node::new(entry);
        self.inner.insert_after(node);
    }

    fn insert_before(&mut self, entry: impl Into<(Interval<K>, V)>) {
        let node = Node::new(entry);
        self.inner.insert_before(node);
    }

    fn insert_after_and_move_next(&mut self, entry: impl Into<(Interval<K>, V)>) {
        let node = Node::new(entry);
        self.inner.insert_after(node);
        self.inner.move_next();
        debug_assert!(!self.inner.is_null());
    }

    fn remove(mut self) -> (RawEntry<K, V>, Cursor<'a, K, V>) {
        let node = *self.inner.remove().expect("null cursor");
        (node.into(), Cursor::new(self.inner))
    }

    fn remove_as_output<Out: OutputBuilder<K, V>>(self) -> (Out::Output, Cursor<'a, K, V>) {
        let (node, cursor) = self.remove();
        (Out::from(node), cursor)
    }
}

struct EmptyCursor<'a, K, V> {
    // Must be null.
    inner: CursorMut<'a, NodeAdapter<K, V>>,
}
impl<'a, K, V> EmptyCursor<'a, K, V> {
    fn first(mut self) -> Cursor<'a, K, V> {
        self.inner.move_next();
        Cursor::new(self.inner)
    }

    fn insert(&mut self, entry: impl Into<(Interval<K>, V)>) {
        let node = Node::new(entry);
        self.inner.insert_after(node);
    }
}
