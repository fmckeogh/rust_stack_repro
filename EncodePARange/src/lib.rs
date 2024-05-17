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
use Have52BitPAExt::*;
use Have56BitPAExt::*;
use common::*;
pub fn EncodePARange<T: Tracer>(state: &mut State, tracer: &T, gs_327635: ()) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u8,
        gs_327635: (),
    }
    let fn_state = FunctionState {
        gs_327635,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #90256u : u32
        let s_0_0: u32 = 90256;
        // D s_0_1: read-reg s_0_0:i
        let s_0_1: i128 = {
            let value = state.read_register::<i128>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #32s : i
        let s_0_2: i128 = 32;
        // D s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: u8 = 0;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var return_value:u8
        let s_2_0: u8 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #90256u : u32
        let s_3_0: u32 = 90256;
        // D s_3_1: read-reg s_3_0:i
        let s_3_1: i128 = {
            let value = state.read_register::<i128>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #36s : i
        let s_3_2: i128 = 36;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #1u : u8
        let s_4_0: u8 = 1;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #90256u : u32
        let s_5_0: u32 = 90256;
        // D s_5_1: read-reg s_5_0:i
        let s_5_1: i128 = {
            let value = state.read_register::<i128>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #40s : i
        let s_5_2: i128 = 40;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #2u : u8
        let s_6_0: u8 = 2;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #90256u : u32
        let s_7_0: u32 = 90256;
        // D s_7_1: read-reg s_7_0:i
        let s_7_1: i128 = {
            let value = state.read_register::<i128>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #42s : i
        let s_7_2: i128 = 42;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #3u : u8
        let s_8_0: u8 = 3;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #90256u : u32
        let s_9_0: u32 = 90256;
        // D s_9_1: read-reg s_9_0:i
        let s_9_1: i128 = {
            let value = state.read_register::<i128>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #44s : i
        let s_9_2: i128 = 44;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #4u : u8
        let s_10_0: u8 = 4;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_11_0: const #90256u : u32
        let s_11_0: u32 = 90256;
        // D s_11_1: read-reg s_11_0:i
        let s_11_1: i128 = {
            let value = state.read_register::<i128>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #48s : i
        let s_11_2: i128 = 48;
        // D s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #5u : u8
        let s_12_0: u8 = 5;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #90256u : u32
        let s_13_0: u32 = 90256;
        // D s_13_1: read-reg s_13_0:i
        let s_13_1: i128 = {
            let value = state.read_register::<i128>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #52s : i
        let s_13_2: i128 = 52;
        // D s_13_3: cmp-eq s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) == (s_13_2));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b15 b14
        if s_13_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Have52BitPAExt(s_14_0)
        let s_14_1: bool = Have52BitPAExt(state, tracer, s_14_0);
        // N s_14_2: assert s_14_1
        let s_14_2: () = assert!(s_14_1);
        // C s_14_3: const #6u : u8
        let s_14_3: u8 = 6;
        // D s_14_4: write-var return_value <= s_14_3
        fn_state.return_value = s_14_3;
        // N s_14_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #90256u : u32
        let s_15_0: u32 = 90256;
        // D s_15_1: read-reg s_15_0:i
        let s_15_1: i128 = {
            let value = state.read_register::<i128>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #56s : i
        let s_15_2: i128 = 56;
        // D s_15_3: cmp-eq s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) == (s_15_2));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Have56BitPAExt(s_16_0)
        let s_16_1: bool = Have56BitPAExt(state, tracer, s_16_0);
        // N s_16_2: assert s_16_1
        let s_16_2: () = assert!(s_16_1);
        // C s_16_3: const #7u : u8
        let s_16_3: u8 = 7;
        // D s_16_4: write-var return_value <= s_16_3
        fn_state.return_value = s_16_3;
        // N s_16_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #6u : u8
        let s_17_0: u8 = 6;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
