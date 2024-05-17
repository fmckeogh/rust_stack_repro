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
use S_set::*;
use CheckAdvSIMDOrVFPEnabled::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d: i64,
    imm32: u32,
    imm64: u64,
    regs: i64,
    single_register: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_312830: i64,
        r: i64,
        advsimd: bool,
        d: i64,
        imm32: u32,
        imm64: u64,
        regs: i64,
        single_register: bool,
    }
    let fn_state = FunctionState {
        advsimd,
        d,
        imm32,
        imm64,
        regs,
        single_register,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: read-var advsimd:u8
        let s_0_1: bool = fn_state.advsimd;
        // D s_0_2: call CheckAdvSIMDOrVFPEnabled(s_0_0, s_0_1)
        let s_0_2: () = CheckAdvSIMDOrVFPEnabled(state, tracer, s_0_0, s_0_1);
        // N s_0_3: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var single_register:u8
        let s_1_0: bool = fn_state.single_register;
        // N s_1_1: branch s_1_0 b6 b2
        if s_1_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i64
        let s_2_0: i64 = 0;
        // C s_2_1: const #1s : i
        let s_2_1: i128 = 1;
        // D s_2_2: read-var regs:i64
        let s_2_2: i64 = fn_state.regs;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: sub s_2_3 s_2_1
        let s_2_4: i128 = ((s_2_3) - (s_2_1));
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // D s_2_6: write-var gs#312830 <= s_2_5
        fn_state.gs_312830 = s_2_5;
        // D s_2_7: write-var r <= s_2_0
        fn_state.r = s_2_0;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var r:i64
        let s_3_0: i64 = fn_state.r;
        // D s_3_1: read-var gs#312830:i64
        let s_3_1: i64 = fn_state.gs_312830;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var r:i64
        let s_4_2: i64 = fn_state.r;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: add s_4_1 s_4_3
        let s_4_4: i128 = (s_4_1 + s_4_3);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var imm64:u64
        let s_4_7: u64 = fn_state.imm64;
        // D s_4_8: call D_set(s_4_6, s_4_7)
        let s_4_8: () = D_set(state, tracer, s_4_6, s_4_7);
        // D s_4_9: read-var r:i64
        let s_4_9: i64 = fn_state.r;
        // C s_4_10: const #1s : i64
        let s_4_10: i64 = 1;
        // D s_4_11: add s_4_9 s_4_10
        let s_4_11: i64 = (s_4_9 + s_4_10);
        // D s_4_12: write-var r <= s_4_11
        fn_state.r = s_4_11;
        // N s_4_13: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var d:i64
        let s_6_0: i64 = fn_state.d;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var imm32:u32
        let s_6_2: u32 = fn_state.imm32;
        // D s_6_3: call S_set(s_6_1, s_6_2)
        let s_6_3: () = S_set(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
}
