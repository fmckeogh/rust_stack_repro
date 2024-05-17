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
use UsingAArch32::*;
use common::*;
pub fn ThisInstrAddr<T: Tracer>(state: &mut State, tracer: &T, N: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        Nshadow_55: i128,
        gs_4456: bool,
        gs_4457: bool,
        N: i128,
    }
    let fn_state = FunctionState {
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i
        let s_0_0: i128 = fn_state.N;
        // D s_0_1: write-var Nshadow#55 <= s_0_0
        fn_state.Nshadow_55 = s_0_0;
        // C s_0_2: const #64s : i
        let s_0_2: i128 = 64;
        // D s_0_3: read-var Nshadow#55:i
        let s_0_3: i128 = fn_state.Nshadow_55;
        // D s_0_4: cmp-eq s_0_3 s_0_2
        let s_0_4: bool = ((s_0_3) == (s_0_2));
        // N s_0_5: branch s_0_4 b6 b1
        if s_0_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #32s : i
        let s_1_0: i128 = 32;
        // D s_1_1: read-var Nshadow#55:i
        let s_1_1: i128 = fn_state.Nshadow_55;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b5 b2
        if s_1_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#4456 <= s_2_0
        fn_state.gs_4456 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var gs#4456:u8
        let s_3_0: bool = fn_state.gs_4456;
        // D s_3_1: write-var gs#4457 <= s_3_0
        fn_state.gs_4457 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#4457:u8
        let s_4_0: bool = fn_state.gs_4457;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #1s : i
        let s_4_2: i128 = 1;
        // D s_4_3: read-var Nshadow#55:i
        let s_4_3: i128 = fn_state.Nshadow_55;
        // D s_4_4: sub s_4_3 s_4_2
        let s_4_4: i128 = ((s_4_3) - (s_4_2));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #12744u : u32
        let s_4_7: u32 = 12744;
        // D s_4_8: read-reg s_4_7:u64
        let s_4_8: u64 = {
            let value = state.read_register::<u64>(s_4_7 as isize);
            tracer.read_register(s_4_7 as isize, value);
            value
        };
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 64u16);
        // D s_4_10: cast zx s_4_5 -> i
        let s_4_10: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_11: const #1s : i64
        let s_4_11: i64 = 1;
        // C s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: sub s_4_10 s_4_6
        let s_4_13: i128 = ((s_4_10) - (s_4_6));
        // D s_4_14: add s_4_13 s_4_12
        let s_4_14: i128 = (s_4_13 + s_4_12);
        // D s_4_15: bit-extract s_4_9 s_4_6 s_4_14
        let s_4_15: Bits = (Bits::new(
            ((s_4_9) >> (s_4_6)).value(),
            u16::try_from(s_4_14).unwrap(),
        ));
        // N s_4_16: return s_4_15
        return s_4_15;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call UsingAArch32(s_5_0)
        let s_5_1: bool = UsingAArch32(state, tracer, s_5_0);
        // D s_5_2: write-var gs#4456 <= s_5_1
        fn_state.gs_4456 = s_5_1;
        // N s_5_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#4457 <= s_6_0
        fn_state.gs_4457 = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
