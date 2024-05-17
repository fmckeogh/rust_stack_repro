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
use ThisInstrLength::*;
use u__id::*;
use fdiv_int::*;
use common::*;
pub fn NextInstrAddr<T: Tracer>(state: &mut State, tracer: &T, N: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_331136: bool,
        Nshadow_8002: i128,
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
        // D s_0_1: write-var Nshadow#8002 <= s_0_0
        fn_state.Nshadow_8002 = s_0_0;
        // D s_0_2: read-var Nshadow#8002:i
        let s_0_2: i128 = fn_state.Nshadow_8002;
        // D s_0_3: call __id(s_0_2)
        let s_0_3: i128 = u__id(state, tracer, s_0_2);
        // C s_0_4: const #1s : i
        let s_0_4: i128 = 1;
        // D s_0_5: sub s_0_3 s_0_4
        let s_0_5: i128 = ((s_0_3) - (s_0_4));
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // D s_0_7: cmp-le s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) <= (s_0_5));
        // N s_0_8: branch s_0_7 b3 b1
        if s_0_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#331136 <= s_1_0
        fn_state.gs_331136 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#331136:u8
        let s_2_0: bool = fn_state.gs_331136;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call ThisInstrLength(s_2_2)
        let s_2_3: i128 = ThisInstrLength(state, tracer, s_2_2);
        // C s_2_4: const #8s : i
        let s_2_4: i128 = 8;
        // S s_2_5: call fdiv_int(s_2_3, s_2_4)
        let s_2_5: i128 = fdiv_int(state, tracer, s_2_3, s_2_4);
        // C s_2_6: const #12744u : u32
        let s_2_6: u32 = 12744;
        // D s_2_7: read-reg s_2_6:u64
        let s_2_7: u64 = {
            let value = state.read_register::<u64>(s_2_6 as isize);
            tracer.read_register(s_2_6 as isize, value);
            value
        };
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 64u16);
        // S s_2_9: cast cvt s_2_5 -> bv
        let s_2_9: Bits = Bits::new(s_2_5 as u128, 128);
        // D s_2_10: add s_2_8 s_2_9
        let s_2_10: Bits = (s_2_8 + s_2_9);
        // D s_2_11: cast reint s_2_10 -> u64
        let s_2_11: u64 = (s_2_10.value() as u64);
        // C s_2_12: const #1s : i
        let s_2_12: i128 = 1;
        // D s_2_13: read-var Nshadow#8002:i
        let s_2_13: i128 = fn_state.Nshadow_8002;
        // D s_2_14: sub s_2_13 s_2_12
        let s_2_14: i128 = ((s_2_13) - (s_2_12));
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // D s_2_17: cast zx s_2_11 -> bv
        let s_2_17: Bits = Bits::new(s_2_11 as u128, 64u16);
        // D s_2_18: cast zx s_2_15 -> i
        let s_2_18: i128 = (i128::try_from(s_2_15).unwrap());
        // C s_2_19: const #1s : i64
        let s_2_19: i64 = 1;
        // C s_2_20: cast zx s_2_19 -> i
        let s_2_20: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_21: sub s_2_18 s_2_16
        let s_2_21: i128 = ((s_2_18) - (s_2_16));
        // D s_2_22: add s_2_21 s_2_20
        let s_2_22: i128 = (s_2_21 + s_2_20);
        // D s_2_23: bit-extract s_2_17 s_2_16 s_2_22
        let s_2_23: Bits = (Bits::new(
            ((s_2_17) >> (s_2_16)).value(),
            u16::try_from(s_2_22).unwrap(),
        ));
        // N s_2_24: return s_2_23
        return s_2_23;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var Nshadow#8002:i
        let s_3_0: i128 = fn_state.Nshadow_8002;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #1s : i
        let s_3_2: i128 = 1;
        // D s_3_3: sub s_3_1 s_3_2
        let s_3_3: i128 = ((s_3_1) - (s_3_2));
        // C s_3_4: const #64s : i
        let s_3_4: i128 = 64;
        // D s_3_5: cmp-lt s_3_3 s_3_4
        let s_3_5: bool = ((s_3_3) < (s_3_4));
        // D s_3_6: write-var gs#331136 <= s_3_5
        fn_state.gs_331136 = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
