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
use Elem_read::*;
use R_set::*;
use D_read::*;
use CheckAdvSIMDOrVFPEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_sr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    esize: i64,
    index: i128,
    n: i64,
    t: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_7564: i64,
        advsimd: bool,
        esize: i64,
        index: i128,
        n: i64,
        t: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        advsimd,
        esize,
        index,
        n,
        t,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#7564 <= s_0_2
        fn_state.esizeshadow_7564 = s_0_2;
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // D s_0_5: read-var advsimd:u8
        let s_0_5: bool = fn_state.advsimd;
        // D s_0_6: call CheckAdvSIMDOrVFPEnabled(s_0_4, s_0_5)
        let s_0_6: () = CheckAdvSIMDOrVFPEnabled(state, tracer, s_0_4, s_0_5);
        // N s_0_7: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var is_unsigned:u8
        let s_1_0: bool = fn_state.is_unsigned;
        // N s_1_1: branch s_1_0 b3 b2
        if s_1_0 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call D_read(s_2_1)
        let s_2_2: u64 = D_read(state, tracer, s_2_1);
        // D s_2_3: read-var esizeshadow#7564:i64
        let s_2_3: i64 = fn_state.esizeshadow_7564;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // D s_2_6: cast zx s_2_2 -> bv
        let s_2_6: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_7: cast zx s_2_5 -> i
        let s_2_7: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_8: read-var index:i
        let s_2_8: i128 = fn_state.index;
        // D s_2_9: call Elem_read(s_2_6, s_2_8, s_2_7)
        let s_2_9: Bits = Elem_read(state, tracer, s_2_6, s_2_8, s_2_7);
        // C s_2_10: const #32s : i
        let s_2_10: i128 = 32;
        // D s_2_11: bits-cast sx s_2_9 -> bv length s_2_10
        let s_2_11: Bits = s_2_9.sign_extend(s_2_10);
        // D s_2_12: cast reint s_2_11 -> u32
        let s_2_12: u32 = (s_2_11.value() as u32);
        // D s_2_13: read-var t:i64
        let s_2_13: i64 = fn_state.t;
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: call R_set(s_2_14, s_2_12)
        let s_2_15: () = R_set(state, tracer, s_2_14, s_2_12);
        // N s_2_16: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // D s_3_3: read-var esizeshadow#7564:i64
        let s_3_3: i64 = fn_state.esizeshadow_7564;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_2 -> bv
        let s_3_6: Bits = Bits::new(s_3_2 as u128, 64u16);
        // D s_3_7: cast zx s_3_5 -> i
        let s_3_7: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_8: read-var index:i
        let s_3_8: i128 = fn_state.index;
        // D s_3_9: call Elem_read(s_3_6, s_3_8, s_3_7)
        let s_3_9: Bits = Elem_read(state, tracer, s_3_6, s_3_8, s_3_7);
        // C s_3_10: const #32s : i
        let s_3_10: i128 = 32;
        // D s_3_11: bits-cast zx s_3_9 -> bv length s_3_10
        let s_3_11: Bits = s_3_9.zero_extend(s_3_10);
        // D s_3_12: cast reint s_3_11 -> u32
        let s_3_12: u32 = (s_3_11.value() as u32);
        // D s_3_13: read-var t:i64
        let s_3_13: i64 = fn_state.t;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: call R_set(s_3_14, s_3_12)
        let s_3_15: () = R_set(state, tracer, s_3_14, s_3_12);
        // N s_3_16: return
        return;
    }
}
