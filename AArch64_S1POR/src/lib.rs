#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Mk_S1PORType::*;
use common::*;
pub fn AArch64_S1POR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        ga_12754: ProductType5c790c8ef59cc8b2,
        return_value: ProductType5c790c8ef59cc8b2,
        regime: u32,
    }
    let fn_state = FunctionState {
        regime,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var regime:u32
        let s_0_1: u32 = fn_state.regime;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_1_0: const #14200u : u32
        let s_1_0: u32 = 14200;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call Mk_S1PORType(s_1_1)
        let s_1_2: ProductType5c790c8ef59cc8b2 = Mk_S1PORType(state, tracer, s_1_1);
        // D s_1_3: write-var return_value <= s_1_2
        fn_state.return_value = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var return_value:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_4_0: const #15336u : u32
        let s_4_0: u32 = 15336;
        // D s_4_1: read-reg s_4_0:u64
        let s_4_1: u64 = {
            let value = state.read_register::<u64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call Mk_S1PORType(s_4_1)
        let s_4_2: ProductType5c790c8ef59cc8b2 = Mk_S1PORType(state, tracer, s_4_1);
        // D s_4_3: write-var return_value <= s_4_2
        fn_state.return_value = s_4_2;
        // N s_4_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_6_0: const #15336u : u32
        let s_6_0: u32 = 15336;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call Mk_S1PORType(s_6_1)
        let s_6_2: ProductType5c790c8ef59cc8b2 = Mk_S1PORType(state, tracer, s_6_1);
        // D s_6_3: write-var return_value <= s_6_2
        fn_state.return_value = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: read-var regime:u32
        let s_7_1: u32 = fn_state.regime;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_8_0: const #13592u : u32
        let s_8_0: u32 = 13592;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call Mk_S1PORType(s_8_1)
        let s_8_2: ProductType5c790c8ef59cc8b2 = Mk_S1PORType(state, tracer, s_8_1);
        // D s_8_3: write-var return_value <= s_8_2
        fn_state.return_value = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_9_0: read-var ga#12754:struct
        let s_9_0: ProductType5c790c8ef59cc8b2 = fn_state.ga_12754;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
