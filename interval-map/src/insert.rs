use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;

use intrusive_collections::Bound;
use intrusive_collections::rbtree::CursorMut;

use crate::interval::{Endpoint, Interval, IntervalLike as _};
use crate::{IntervalAndValue, IntervalMap, Node, NodeAdapter, ValuesBetweenExist};

pub struct Insert<'a, K, V>
where
    K: ValuesBetweenExist + 'static,
    V: PartialEq,
{
    state: Option<State<'a, K, V>>,
}
impl<'a, K, V> Insert<'a, K, V>
where
    K: ValuesBetweenExist + 'static,
    V: PartialEq,
{
    pub(crate) fn new(map: &'a mut IntervalMap<K, V>, inserting: IntervalAndValue<K, V>) -> Self {
        Insert {
            state: Some(State::State0 { map, inserting }),
        }
    }

    fn next_item<H: OutputHandler<K, V>>(&mut self) -> Option<H::Output> {
        self.state.take().and_then(|state| {
            match state {
                State::State0 { map, inserting } => {
                    if inserting.interval.is_empty() {
                        todo!()
                    }

                    /*
                    inserting:  {IL, IR} -> IV
                    There MAY be nodes N where N.left <= IL and N.right is in [IL, IR].
                    There MAY be nodes N where N.left is in (IL, IR].
                    */

                    let bound = Bound::Included(&inserting.left_relative());
                    let mut cursor = map.tree.upper_bound_mut(bound);
                    if let Some(mut node_ptr) = cursor.get_ptr() {
                        let node = unsafe { node_ptr.as_mut() };
                        debug_assert!(node.left_relative() <= inserting.left_relative());
                        /*
                            inserting:     {IL, ⋯IR} -> IV
                            node:      {NL,     ⋯NR} -> NV
                            Where NL <= IL; NR <?> IL; NR <?> IR; IV =?= NV.
                            There are NO other nodes N where N.left is in (NL, IL].
                            There MAY be other nodes N where N.left is in (IL, IR].
                        */

                        if node.right_relative() < inserting.left_relative() {
                            // NR < IL
                            /*
                                inserting:         {IL, IR} -> IV
                                node:      {NL, NR}         -> NV
                                Where NL <= NR < IL <= IR; IV =?= NV.
                                There MAY be other nodes N where N.left is in (IL, IR].
                            */
                            todo!()
                        } else {
                            // NR >= IL
                            /*
                                inserting:     {IL, ⋯IR} -> IV
                                node:      {NL,     NR} -> NV
                                Where NL <= IL <= NR; NR <?> IR; IV =?= NV.
                                There MAY be other nodes N where N.left is in (IL, IR].
                            */

                            let rights_cmp = node.right_relative().cmp(&inserting.right_relative());
                            if rights_cmp.is_le() {
                                // NR <= IR
                                /*
                                    inserting:     {IL,     IR} -> IV
                                    node:      {NL,     NR}     -> NV
                                    Where NL <= IL <= NR <= IR; IV =?= NV.
                                    There MAY be other nodes N where N.left is in (NR, IR].
                                */

                                let output = if node.value == inserting.value {
                                    // NV = IV
                                    /*
                                        inserting:     {IL,     IR} -> V
                                        node:      {NL,     NR}     -> V
                                        Where NL <= IL <= NR <= IR.
                                        There MAY be other nodes N where N.left is in (NR, IR].
                                    */
                                    let output = H::output(
                                        inserting.interval.left,
                                        node.replace_right(inserting.interval.right),
                                        inserting.value,
                                    );
                                    /*
                                        node:      {NL,         IR} -> V
                                        output:        {IL, NR}     -> V
                                    */
                                    output
                                } else {
                                    // NV != IV
                                    /*
                                        inserting:     {IL,     IR} -> IV
                                        node:      {NL,     NR}     -> NV
                                        Where NL <= IL <= NR <= IR; IV != NV.
                                        There MAY be other nodes N where N.left is in (NR, IR].
                                    */

                                    // {NL, IL@
                                    match emptiness_right_excluded(node.left(), inserting.left()) {
                                        EmptinessRightExcluded::Empty(before) => {
                                            /*
                                                inserting:     {IL,     IR} -> IV
                                                node:      {NL,     NR}     -> NV
                                                Where NL <= IL <= NR <= IR; {NL, IL@ is empty; IV != NV.
                                                There MAY be other nodes N where N.left is in (NR, IR].
                                            */
                                            let output = H::output(
                                                inserting.interval.left,
                                                node.replace_right(inserting.interval.right),
                                                node.replace_value(inserting.value),
                                            );
                                            /*
                                                node:      {NL,         IR} -> IV
                                                output:        {IL, NR}     -> NV
                                            */
                                            output
                                        }
                                        EmptinessRightExcluded::NonEmpty(before) => {
                                            /*
                                                inserting:     {IL,     IR} -> IV
                                                node:      {NL,     NR}     -> NV
                                                Where NL <= IL <= NR <= IR; {NL, IL@ is NOT empty; IV != NV.
                                                There MAY be other nodes N where N.left is in (NR, IR].
                                            */
                                            todo!()
                                        }
                                        EmptinessRightExcluded::RightInfinity(left) => {
                                            // IL = -inf
                                            /*
                                                inserting: (-inf,     IR} -> IV
                                                node:      (-inf, NR}     -> NV
                                                Where -inf <= NR <= IR; IV != NV.
                                                There MAY be other nodes N where N.left is in (NR, IR].
                                            */
                                            todo!()
                                        }
                                    }
                                };

                                if rights_cmp.is_lt() {
                                    // NR < IR
                                    todo!("other nodes")
                                    // self.state = State::OtherNodes;
                                }

                                Some(output)
                            } else {
                                // NR > IR
                                /*
                                    inserting:     {IL, IR}     -> IV
                                    node:      {NL,         NR} -> NV
                                    Where NL <= IL <= IR < NR; IV =?= NV.
                                    There are NO other nodes N where N.left is in (IL, IR].
                                */
                                todo!()
                            }
                        }
                    } else {
                        /*
                            inserting: {IL, IR} -> IV
                            There are NO node N where N.left <= IL and N.right is in [IL, IR].
                            There MAY be nodes N where N.left is in (IL, IR].
                        */

                        let mut cursor = map.tree.front_mut();
                        if let Some(mut node_ptr) = cursor.get_ptr() {
                            let node = unsafe { node_ptr.as_mut() };
                            debug_assert!(node.left_relative() > inserting.left_relative());
                            /*
                                inserting: {IL,     ⋯IR} -> IV
                                node:          {NL, ⋯NR} -> NV
                                Where IL < NL; IR <?> NL; NR <?> IR; IV =?= NV.
                                There MAY be other nodes N where N.left is in (NR, IR].
                            */
                            todo!()
                        } else {
                            /*
                                inserting: {IL, IR} -> IV
                                There are NO nodes.
                            */
                            map.tree.insert(Node::new(inserting));
                            None
                        }
                    }
                }
            }
        })
    }
}
impl<'a, K, V> Iterator for Insert<'a, K, V>
where
    K: ValuesBetweenExist + 'static,
    V: PartialEq,
{
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item::<ReturnItem>()
    }
}
impl<'a, K, V> Drop for Insert<'a, K, V>
where
    K: ValuesBetweenExist + 'static,
    V: PartialEq,
{
    fn drop(&mut self) {
        while self.next_item::<ReturnNothing>().is_some() {}
    }
}

enum State<'a, K, V> {
    State0 {
        map: &'a mut IntervalMap<K, V>,
        inserting: IntervalAndValue<K, V>,
    },
}

trait OutputHandler<K, V> {
    type Output;

    fn output(left: Endpoint<K>, right: Endpoint<K>, value: V) -> Self::Output;
}

struct ReturnItem;
impl<K, V> OutputHandler<K, V> for ReturnItem {
    type Output = (Interval<K>, V);

    fn output(left: Endpoint<K>, right: Endpoint<K>, value: V) -> Self::Output {
        todo!()
    }
}

struct ReturnNothing;
impl<K, V> OutputHandler<K, V> for ReturnNothing {
    type Output = ();

    fn output(left: Endpoint<K>, right: Endpoint<K>, value: V) -> Self::Output {
        ()
    }
}

pub(crate) fn insert_<K, V>(
    map: &mut IntervalMap<K, V>,
    interval: Interval<K>,
    value: V,
) -> Vec<(Interval<K>, V)>
where
    K: ValuesBetweenExist + Clone + Debug + 'static,
    V: PartialEq + Clone + Debug,
{
    let interval = interval.into_interval();
    if interval.is_empty() {
        return Vec::new();
    }
    let mut inserting = IntervalAndValue { interval, value };

    /*
       inserting:  {IL, ···IR} -> IV
       There MAY be nodes N where N.left <= IL and N.right is in [IL, IR].
       There MAY be nodes N where N.left is in (IL, IR].
    */

    let bound = Bound::Included(&inserting.left_relative());
    let mut cursor = map.tree.upper_bound_mut(bound);

    if let Some(mut node_ptr) = cursor.get_ptr() {
        let node = unsafe { node_ptr.as_mut() };
        debug_assert!(node.left_relative() <= inserting.left_relative());
        /*
            inserting:     {IL, ···IR} -> IV
            node:      {NL,     ···NR} -> NV
            Where NL <= IL; NR <?> IL; NR <?> IR; IV =?= NV.
            There are NO other nodes N where N.left is in (NL, IL].
            There MAY be other nodes N where N.left is in (IL, IR].
        */

        if node.right_relative() >= inserting.left_relative() {
            // IL <= NR
            /*
                inserting:     {IL, ···IR} -> IV
                node:      {NL,     NR}    -> NV
                Where NL <= IL <= NR <?> IR; IV =?= NV.
                There MAY be other nodes N where N.left is in (NR, IR].
            */

            let rights_cmp = node.right_relative().cmp(&inserting.right_relative());
            if rights_cmp.is_le() {
                // NR <= IR
                /*
                    inserting:     {IL,     IR} -> IV
                    node:      {NL,     NR}     -> NV
                    Where NL <= IL <= NR <= IR; IV =?= NV.
                    If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                */

                let mut previous_entries = Vec::new();

                if node.value == inserting.value {
                    // NV = IV
                    /*
                        inserting:     {IL,     IR} -> V
                        In map:    {NL,     NR}     -> V   <=node
                        Where NL <= IL <= NR <= IR.
                        If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                    */

                    // Extend the existing node.
                    let previous_right = node.replace_right(inserting.interval.right);
                    /*
                        inserting:     {IL,      _} -> V
                        In map:    {NL,         IR} -> V   <=node
                        previous:      {_,  NR}     -> _
                    */
                    let previous = IntervalAndValue {
                        interval: Interval {
                            left: inserting.interval.left,
                            right: previous_right,
                        },
                        value: inserting.value,
                    };
                    /*
                        In map:    {NL,         IR} -> V   <=node
                        previous:      {IL, NR}     -> V
                    */

                    previous_entries.push(previous.into_tuple());
                } else {
                    // NV != IV
                    /*
                        inserting:     {IL,     IR} -> IV
                        node:      {NL,     NR}     -> NV
                        Where NL <= IL <= NR <= IR; IV != NV.
                        If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                    */

                    // before = {NL, IL@
                    match emptiness_right_excluded(node.left(), inserting.left()) {
                        EmptinessRightExcluded::Empty(before) => {
                            /*
                                inserting:     {IL,     IR} -> IV
                                In map:    {NL,     NR}     -> NV   <=node
                                before:    {NL, IL@
                                Where NL <= IL <= NR <= IR; {NL, IL@ IS empty; IV != NV.
                                If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                            */
                            let previous = node.replace(inserting);
                            /*
                                In map:        {IL,     IR} -> IV   <=node
                                previous:  {NL,     NR}     -> NV
                            */
                            previous_entries.push(previous);
                        }
                        EmptinessRightExcluded::NonEmpty(before) => {
                            /*
                                inserting:     {IL,     IR} -> IV
                                In map:    {NL,     NR}     -> NV   <=node/cursor
                                before:    {NL, IL@
                                Where NL <= IL <= NR <= IR; {NL, IL@ is NOT empty; IV != NV.
                                If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                            */

                            // Shorten the existing entry.
                            let previous_right = node.replace_right(before.right.cloned());
                            /*
                                inserting:     {IL,     IR} -> IV
                                In map:    {NL, IL@         -> NV   <=node/cursor
                                previous:      {_,  NR}     -> _
                            */
                            let previous = IntervalAndValue {
                                interval: Interval {
                                    left: inserting.left().clone(),
                                    right: previous_right,
                                },
                                value: node.value.clone(),
                            };
                            /*
                                inserting:     {IL,     IR} -> IV
                                In map:    {NL, IL@         -> NV   <=node/cursor
                                previous:      {IL, NR}     -> NV
                            */
                            cursor.insert_after(Node::new(inserting));
                            /*
                                In map:    {NL, IL@         -> NV   <=node/cursor
                                               {IL,     IR} -> IV
                                previous:      {IL, NR}     -> NV
                            */
                            cursor.move_next();
                            /*
                                In map:    {NL, IL@         -> NV   <=node
                                               {IL,     IR} -> IV   <=cursor
                                previous:      {IL, NR}     -> NV
                            */
                            previous_entries.push(previous.into_tuple());
                        }
                        EmptinessRightExcluded::RightInfinity(left) => {
                            // IL = -inf
                            /*
                                inserting: (-inf,     IR} -> IV
                                node:      (-inf, NR}     -> NV
                                Where -inf <= NR <= IR; IV != NV.
                                If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                            */
                            todo!("{map:?} + {inserting:?}")
                        }
                    }
                }

                if rights_cmp.is_lt() {
                    // NR < IR
                    /*
                        cursor: {_, ···IR} -> IV
                        There MAY be other nodes N where N.left <= IR.
                    */
                    remove_overridden_nodes(cursor, &mut previous_entries);
                }

                previous_entries
            } else {
                // NR > IR
                /*
                    inserting:     {IL, IR}     -> IV
                    node:      {NL,         NR} -> NV
                    Where NL <= IL <= IR < NR; IV =?= NV.
                    There are NO other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserting.value {
                    // NV = IV
                    /*
                        inserting:     {IL, IR}     -> V
                        In map:    {NL,         NR} -> V   <=node
                        Where NL <= IL <= IR < NR.
                    */
                    let previous = inserting;
                    /*
                        In map:    {NL,         NR} -> V   <=node
                        previous:      {IL, IR}     -> V
                    */
                    Vec::from([previous.into_tuple()])
                } else {
                    // NV != IV
                    /*
                        inserting:     {IL, IR}     -> IV
                        node:      {NL,         NR} -> NV
                        Where NL <= IL <= IR < NR; IV != NV.
                    */

                    // before = {NL, IL@
                    match emptiness_right_excluded(node.left(), inserting.left()) {
                        EmptinessRightExcluded::Empty(before) => {
                            /*
                                inserting:     {IL, IR}     -> IV
                                node:      {NL,         NR} -> NV
                                before:    {NL, IL@
                                Where NL <= IL <= IR < NR; {NL, IL@ IS empty; IV != NV.
                            */

                            // after = @IR, NR}
                            match emptiness_left_excluded(inserting.right(), node.right()) {
                                EmptinessLeftExcluded::Empty(after) => {
                                    /*
                                        inserting:     {IL, IR}     -> IV
                                        In map:    {NL,         NR} -> NV   <=node
                                        after:             @IR, NR}
                                        Where NL <= IL <= IR < NR; {NL, IL@ IS empty; @IR, NR} IS empty; IV != NV.
                                    */
                                    let previous = node.replace(inserting);
                                    /*
                                        In map:        {IL, IR}     -> IV   <=node
                                        previous:  {NL,         NR} -> NV
                                    */
                                    Vec::from([previous])
                                }
                                EmptinessLeftExcluded::NonEmpty(after) => {
                                    /*
                                        inserting:     {IL, IR}     -> IV
                                        In map:    {NL,         NR} -> NV   <=node
                                        after:             @IR, NR}
                                        Where NL <= IL <= IR < NR; {NL, IL@ IS empty; @IR, NR} is NOT empty; IV != NV.
                                    */

                                    // Shorten the existing entry.
                                    let previous_left = node.replace_left(after.left.cloned());
                                    /*
                                        inserting:     {IL, IR}     -> IV
                                        In map:            @IR, NR} -> NV   <=node
                                        previous:  {NL,      _}     -> _
                                    */
                                    let previous = IntervalAndValue {
                                        interval: Interval {
                                            left: previous_left,
                                            right: inserting.right().clone(),
                                        },
                                        value: node.value.clone(),
                                    };
                                    /*
                                        inserting:     {IL, IR}     -> IV
                                        In map:            @IR, NR} -> NV   <=node
                                        previous:  {NL,     IR}     -> NV
                                    */
                                    cursor.insert_before(Node::new(inserting));
                                    /*
                                        In map:        {IL, IR}     -> IV
                                                            @IR, NR} -> NV   <=node
                                        previous:  {NL,     IR}     -> NV
                                    */
                                    Vec::from([previous.into_tuple()])
                                }
                                EmptinessLeftExcluded::LeftInfinity(right) => {
                                    // IR = +inf
                                    /*
                                        inserting:     {IL, +inf)   -> IV
                                        node:      {NL,         NR} -> NV
                                        Where NL <= IL <= +inf < NR (!!!); {NL, IL@ IS empty; IV != NV.
                                    */
                                    unreachable!()
                                }
                            }
                        }
                        EmptinessRightExcluded::NonEmpty(before) => {
                            /*
                                inserting:     {IL, IR}     -> IV
                                node:      {NL,         NR} -> NV
                                before:    {NL, IL@
                                Where NL <= IL <= IR < NR; {NL, IL@ is NOT empty; IV != NV.
                            */
                            todo!("{map:?} + {inserting:?}")
                        }
                        EmptinessRightExcluded::RightInfinity(left) => {
                            // IL = -inf
                            /*
                                inserting: (-inf, IR}     -> IV
                                node:      (-inf,     NR} -> NV
                                Where -inf <= IR < NR; IV != NV.
                            */
                            todo!("{map:?} + {inserting:?}")
                        }
                    }
                }
            }
        } else {
            // NR < IL
            /*
                inserting:         {IL, IR} -> IV
                node:      {NL, NR}         -> NV
                Where NL <= NR < IL <= IR; IV =?= NV.
                There are NO other nodes N where N.left is in (NL, IL].
                There MAY be other nodes N where N.left is in (IL, IR].
            */

            // gap = @NR, IL@
            if node.value == inserting.value
                && let EmptinessBothExcluded::Empty(gap) =
                    emptiness_both_excluded(node.right(), inserting.left())
            {
                // IV = NV
                /*
                    inserting:         {IL, IR} -> V
                    In map:    {NL, NR}         -> V   <=node
                    gap:           @NR, IL@
                    Where NL <= NR < IL <= IR; @NR, IL@ IS empty.
                    There MAY be other nodes N where N.left is in (IL, IR].
                */

                // Extend the existing entry.
                node.interval.right = inserting.interval.right;
                /*
                    In map:    {NL, IR} -> V   <=node
                    previous: -
                */

                let mut previous_entries = Vec::new();
                remove_overridden_nodes(cursor, &mut previous_entries);
                previous_entries
            } else {
                // IV != NV or @NR, IL@ is NOT empty
                /*
                    inserting:         {IL, IR} -> IV
                    In map:    {NL, NR}         -> NV   <=node/cursor
                    Where NL <= NR < IL <= IR; IV =?= NV.
                    There MAY be other nodes N where N.left is in (IL, IR].
                */

                cursor.insert_after(Node::new(inserting));
                /*
                    In map:    {NL, NR}         -> NV   <=node/cursor
                                       {IL, IR} -> IV
                    previous: -
                */
                cursor.move_next();
                /*
                    In map:    {NL, NR}         -> NV   <=node
                                       {IL, IR} -> IV   <=cursor
                    previous: -
                */

                let mut previous_entries = Vec::new();
                remove_overridden_nodes(cursor, &mut previous_entries);
                previous_entries
            }
        }
    } else {
        /*
            inserting: {IL, ···IR} -> IV
            There are NO node N where N.left <= IL and N.right is in [IL, IR].
            There MAY be nodes N where N.left is in (IL, IR].
        */

        // Check if the first interval intersects with the interval being inserted.
        let mut cursor = map.tree.front_mut();

        if let Some(mut node_ptr) = cursor.get_ptr() {
            let node = unsafe { node_ptr.as_mut() };
            debug_assert!(node.left_relative() > inserting.left_relative());
            /*
                inserting: {IL,     ···IR} -> IV
                node:          {NL, ···NR} -> NV
                Where IL < NL; IR <?> NL; NR <?> IR; IV =?= NV.
                There MAY be other nodes N where N.left is in (NR, IR].
            */

            if node.left_relative() <= inserting.right_relative() {
                // NL <= IR
                /*
                    inserting: {IL,     IR}    -> IV
                    In map:        {NL, ···NR} -> NV   <=node
                    Where IL < NL <= IR; NR <?> IR; IV =?= NV.
                    There MAY be other nodes N where N.left is in (NR, IR].
                */

                let rights_cmp = node.right_relative().cmp(&inserting.right_relative());
                if rights_cmp.is_le() {
                    // NR <= IR
                    /*
                        inserting: {IL,         IR} -> IV
                        In map:        {NL, NR}     -> NV   <=node
                        Where IL < NL <= NR <= IR; IV =?= NV.
                        If NR < IR: there MAY be other nodes N where N.left is in (NR, IR].
                    */
                    let previous = node.replace(inserting);
                    /*
                        In map:    {IL,         IR} -> IV   <=node/cursor
                        previous:      {NL, NR}     -> NV
                    */

                    let mut previous_entries = Vec::from([previous]);
                    if rights_cmp.is_lt() {
                        // NR < IR
                        remove_overridden_nodes(cursor, &mut previous_entries);
                    }
                    previous_entries
                } else {
                    // NR > IR
                    /*
                        inserting: {IL,     IR}     -> IV
                        In map:        {NL,     NR} -> NV   <=node
                        Where IL < NL <= IR < NR; IV =?= NV.
                        There are NO other nodes N where N.left is in (NR, IR].
                    */

                    if node.value == inserting.value {
                        // IV = NV
                        /*
                            inserting: {IL,     IR}     -> V
                            In map:        {NL,     NR} -> V   <=node
                        */

                        // Extend the existing entry.
                        node.swap_left(inserting.left_mut());
                        let previous = inserting;
                        /*
                            In map:    {IL,         NR} -> V   <=node
                            previous:      {NL, IR}     -> V
                        */
                        Vec::from([previous.into_tuple()])
                    } else {
                        // IV != NV
                        /*
                            inserting: {IL,     IR}     -> IV
                            node:          {NL,     NR} -> NV
                            Where IL < NL <= IR < NR; IV != NV.
                        */

                        // after = @IR, NR}
                        match emptiness_left_excluded(inserting.right(), node.right()) {
                            EmptinessLeftExcluded::Empty(_) => {
                                /*
                                    inserting: {IL,     IR}     -> IV
                                    In map:        {NL,     NR} -> NV   <=node
                                    after:             @IR, NR}
                                    Where IL < NL <= IR < NR; @IR, NR} IS empty; IV != NV.
                                */
                                let previous = node.replace(inserting);
                                /*
                                    In map:    {IL,     IR}     -> IV
                                    previous:      {NL,     NR} -> NV   <=node
                                */
                                Vec::from([previous])
                            }
                            EmptinessLeftExcluded::NonEmpty(after) => {
                                /*
                                    inserting: {IL,    IR}      -> IV
                                    In map:        {NL,     NR} -> NV   <=node
                                    after:             @IR, NR}
                                    Where IL < NL <= IR < NR; @IR, NR} is NOT empty; IV != NV.
                                */

                                // Shorten the existing entry.
                                let previous_left = node.replace_left(after.left.cloned());
                                /*
                                    inserting: {IL,    IR}     -> IV
                                    In map:           @IR, NR} -> NV   <=node
                                    previous:     {NL,  _}     -> _
                                */
                                let previous = IntervalAndValue {
                                    interval: Interval {
                                        left: previous_left,
                                        right: inserting.right().clone(),
                                    },
                                    value: node.value.clone(),
                                };
                                /*
                                    inserting: {IL,    IR}     -> IV
                                    In map:           @IR, NR} -> NV   <=node
                                    previous:     {NL, IR}     -> NV
                                */
                                cursor.insert_before(Node::new(inserting));
                                /*
                                    In map:    {IL,    IR}     -> IV   <=node
                                                      @IR, NR} -> NV
                                    previous:     {NL, IR}     -> NV
                                */
                                Vec::from([previous.into_tuple()])
                            }
                            EmptinessLeftExcluded::LeftInfinity(_) => {
                                // IR = +inf
                                /*
                                    inserting: {IL,  +inf)   -> IV
                                    node:         {NL,   NR} -> NV
                                    Where IL < NL <= +inf < NR (!!!); IV != NV.
                                */
                                unreachable!();
                            }
                        }
                    }
                }
            } else {
                // NL > IR
                /*
                    inserting: {IL, IR}         -> IV
                    node:              {NL, NR} -> NV
                    Where IL <= IR < NL <= NR; IV =?= NV.
                    There are NO other nodes N where N.left is in (NR, IR].
                */

                if node.value == inserting.value {
                    // IV = NV
                    /*
                        inserting: {IL, IR}         -> V
                        node:              {NL, NR} -> V
                        Where IL <= IR < NL <= NR.
                    */

                    // gap = @IR, NL@
                    match emptiness_both_excluded(inserting.right(), node.left()) {
                        EmptinessBothExcluded::Empty(_) => {
                            /*
                                inserting: {IL, IR}         -> V
                                In map:            {NL, NR} -> V   <=node
                                gap:           @IR, NL@
                                Where IL <= IR < NL <= NR; @IR, NL@ IS empty.
                            */

                            // Extend the existing  node.
                            node.interval.left = inserting.interval.left;
                            /*
                                In map:    {IL, NR} -> V   <=node
                                previous: -
                            */
                            Vec::new()
                        }
                        EmptinessBothExcluded::NonEmpty(_) => {
                            /*
                                inserting: {IL, IR}         -> V
                                In map:            {NL, NR} -> V   <=node
                                gap:           @IR, NL@
                                Where IL <= IR < NL <= NR; @IR, NL@ is NOT empty.
                            */
                            cursor.insert_before(Node::new(inserting));
                            /*
                                In map:    {IL, IR}         -> V
                                                   {NL, NR} -> V   <=node
                                previous: -
                            */
                            Vec::new()
                        }
                        _ => {
                            // IR = +inf or NL = -inf (!!!)
                            unreachable!()
                        }
                    }
                } else {
                    // IV != NV
                    /*
                        inserting: {IL, IR}         -> IV
                        In map:            {NL, NR} -> NV   <=node
                        Where IL <= IR < NL <= NR; IV != NV.
                    */
                    cursor.insert_before(Node::new(inserting));
                    /*
                        In map:    {IL, IR}         -> IV
                                           {NL, NR} -> NV   <=node
                        Previous: -
                    */
                    Vec::new()
                }
            }
        } else {
            /*
                inserting: {IL, IR} -> IV
                In map:    -
            */
            // No elements in the tree.
            map.tree.insert(Node::new(inserting));
            /*
                In map: {IL, IR} -> IV
                previous: -
            */
            Vec::new()
        }
    }
}

enum EmptinessLeftExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    LeftInfinity(Endpoint<&'a K>),
}

fn emptiness_left_excluded<'a, K>(
    left: &'a Endpoint<K>,
    right: &'a Endpoint<K>,
) -> EmptinessLeftExcluded<'a, K>
where
    K: ValuesBetweenExist,
{
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

enum EmptinessRightExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    RightInfinity(Endpoint<&'a K>),
}

fn emptiness_right_excluded<'a, K>(
    left: &'a Endpoint<K>,
    right: &'a Endpoint<K>,
) -> EmptinessRightExcluded<'a, K>
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

enum EmptinessBothExcluded<'a, K> {
    Empty(Interval<&'a K>),
    NonEmpty(Interval<&'a K>),
    LeftInfinity(Endpoint<&'a K>),
    RightInfinity(Endpoint<&'a K>),
    BothInfinity,
}

fn emptiness_both_excluded<'a, K>(
    left: &'a Endpoint<K>,
    right: &'a Endpoint<K>,
) -> EmptinessBothExcluded<'a, K>
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

// Pre-condition: `cursor` points to the node with the inserted interval and value.
fn remove_overridden_nodes<K, V>(
    mut cursor: CursorMut<'_, NodeAdapter<K, V>>,
    previous_entries: &mut Vec<(Interval<K>, V)>,
) {
    let inserted = unsafe { cursor.get_ptr().expect("must not be null").as_mut() };
    /*
        inserted: {_, IR} -> IV   <=cursor
    */

    loop {
        cursor.move_next();
        let Some(node_ptr) = cursor.get_ptr() else {
            // No more nodes.
            break;
        };

        todo!()
    }
}
