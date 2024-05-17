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
use D_set::*;
use Elem_set::*;
use R_read::*;
use D_read::*;
use CheckAdvSIMDOrVFPEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_rs_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d__arg: i64,
    esize: i64,
    index: i128,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7557: i64,
        d: i128,
        advsimd: bool,
        d__arg: i64,
        esize: i64,
        index: i128,
        t: i64,
    }
    let fn_state = FunctionState {
        advsimd,
        d__arg,
        esize,
        index,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#7557 <= s_0_2
        fn_state.esizeshadow_7557 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // D s_0_8: read-var advsimd:u8
        let s_0_8: bool = fn_state.advsimd;
        // D s_0_9: call CheckAdvSIMDOrVFPEnabled(s_0_7, s_0_8)
        let s_0_9: () = CheckAdvSIMDOrVFPEnabled(state, tracer, s_0_7, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var d:i
        let s_1_0: i128 = fn_state.d;
        // D s_1_1: call D_read(s_1_0)
        let s_1_1: u64 = D_read(state, tracer, s_1_0);
        // D s_1_2: read-var esizeshadow#7557:i64
        let s_1_2: i64 = fn_state.esizeshadow_7557;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: read-var t:i64
        let s_1_5: i64 = fn_state.t;
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // D s_1_7: call R_read(s_1_6)
        let s_1_7: u32 = R_read(state, tracer, s_1_6);
        // C s_1_8: const #1s : i
        let s_1_8: i128 = 1;
        // D s_1_9: read-var esizeshadow#7557:i64
        let s_1_9: i64 = fn_state.esizeshadow_7557;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: sub s_1_10 s_1_8
        let s_1_11: i128 = ((s_1_10) - (s_1_8));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // C s_1_13: const #0s : i
        let s_1_13: i128 = 0;
        // D s_1_14: cast zx s_1_7 -> bv
        let s_1_14: Bits = Bits::new(s_1_7 as u128, 32u16);
        // D s_1_15: cast zx s_1_12 -> i
        let s_1_15: i128 = (i128::try_from(s_1_12).unwrap());
        // C s_1_16: const #1s : i64
        let s_1_16: i64 = 1;
        // C s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: sub s_1_15 s_1_13
        let s_1_18: i128 = ((s_1_15) - (s_1_13));
        // D s_1_19: add s_1_18 s_1_17
        let s_1_19: i128 = (s_1_18 + s_1_17);
        // D s_1_20: bit-extract s_1_14 s_1_13 s_1_19
        let s_1_20: Bits = (Bits::new(
            ((s_1_14) >> (s_1_13)).value(),
            u16::try_from(s_1_19).unwrap(),
        ));
        // D s_1_21: cast zx s_1_1 -> bv
        let s_1_21: Bits = Bits::new(s_1_1 as u128, 64u16);
        // D s_1_22: cast zx s_1_4 -> i
        let s_1_22: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_23: read-var index:i
        let s_1_23: i128 = fn_state.index;
        // D s_1_24: call Elem_set(s_1_21, s_1_23, s_1_22, s_1_20)
        let s_1_24: Bits = Elem_set(state, tracer, s_1_21, s_1_23, s_1_22, s_1_20);
        // D s_1_25: cast reint s_1_24 -> u64
        let s_1_25: u64 = (s_1_24.value() as u64);
        // D s_1_26: read-var d:i
        let s_1_26: i128 = fn_state.d;
        // D s_1_27: call D_set(s_1_26, s_1_25)
        let s_1_27: () = D_set(state, tracer, s_1_26, s_1_25);
        // N s_1_28: return
        return;
    }
}
