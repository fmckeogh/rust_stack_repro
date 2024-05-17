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
use zext_subrange::*;
use R_read::*;
use u__id::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_UBFX_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    lsbit: i64,
    msbit: i128,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_304015: bool,
        d: i64,
        lsbit: i64,
        msbit: i128,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        lsbit,
        msbit,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var lsbit:i64
        let s_0_0: i64 = fn_state.lsbit;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call __id(s_0_1)
        let s_0_2: i128 = u__id(state, tracer, s_0_1);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: read-var msbit:i
        let s_0_4: i128 = fn_state.msbit;
        // D s_0_5: call __id(s_0_4)
        let s_0_5: i128 = u__id(state, tracer, s_0_4);
        // D s_0_6: cast zx s_0_3 -> i
        let s_0_6: i128 = (i128::try_from(s_0_3).unwrap());
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
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#304015 <= s_1_0
        fn_state.gs_304015 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#304015:u8
        let s_2_0: bool = fn_state.gs_304015;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var n:i64
        let s_2_2: i64 = fn_state.n;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call R_read(s_2_3)
        let s_2_4: u32 = R_read(state, tracer, s_2_3);
        // C s_2_5: const #32s : i
        let s_2_5: i128 = 32;
        // D s_2_6: cast zx s_2_4 -> bv
        let s_2_6: Bits = Bits::new(s_2_4 as u128, 32u16);
        // D s_2_7: read-var lsbit:i64
        let s_2_7: i64 = fn_state.lsbit;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: read-var msbit:i
        let s_2_9: i128 = fn_state.msbit;
        // D s_2_10: call zext_subrange(s_2_5, s_2_6, s_2_9, s_2_8)
        let s_2_10: Bits = zext_subrange(state, tracer, s_2_5, s_2_6, s_2_9, s_2_8);
        // D s_2_11: cast reint s_2_10 -> u32
        let s_2_11: u32 = (s_2_10.value() as u32);
        // D s_2_12: read-var d:i64
        let s_2_12: i64 = fn_state.d;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: call R_set(s_2_13, s_2_11)
        let s_2_14: () = R_set(state, tracer, s_2_13, s_2_11);
        // N s_2_15: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var msbit:i
        let s_3_0: i128 = fn_state.msbit;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #32s : i
        let s_3_2: i128 = 32;
        // D s_3_3: cmp-lt s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) < (s_3_2));
        // D s_3_4: write-var gs#304015 <= s_3_3
        fn_state.gs_304015 = s_3_3;
        // N s_3_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
