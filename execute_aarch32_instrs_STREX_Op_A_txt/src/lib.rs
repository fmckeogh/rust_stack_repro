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
use R_read::*;
use AArch32_ExclusiveMonitorsPass::*;
use R_set::*;
use MemA_set::*;
use common::*;
pub fn execute_aarch32_instrs_STREX_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm32: u32,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        address: u32,
        ga_347974: bool,
        d: i64,
        imm32: u32,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        d,
        imm32,
        n,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: read-var imm32:u32
        let s_0_4: u32 = fn_state.imm32;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_6: add s_0_3 s_0_5
        let s_0_6: Bits = (s_0_3 + s_0_5);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: write-var address <= s_0_7
        fn_state.address = s_0_7;
        // C s_0_9: const #4s : i
        let s_0_9: i128 = 4;
        // D s_0_10: read-var address:u32
        let s_0_10: u32 = fn_state.address;
        // D s_0_11: call AArch32_ExclusiveMonitorsPass(s_0_10, s_0_9)
        let s_0_11: bool = AArch32_ExclusiveMonitorsPass(state, tracer, s_0_10, s_0_9);
        // D s_0_12: write-var ga#347974 <= s_0_11
        fn_state.ga_347974 = s_0_11;
        // N s_0_13: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var ga#347974:u8
        let s_1_0: bool = fn_state.ga_347974;
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
        // C s_2_0: const #32s : i
        let s_2_0: i128 = 32;
        // C s_2_1: const #1u : u8
        let s_2_1: bool = true;
        // C s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // D s_2_3: bits-cast zx s_2_2 -> bv length s_2_0
        let s_2_3: Bits = s_2_2.zero_extend(s_2_0);
        // D s_2_4: cast reint s_2_3 -> u32
        let s_2_4: u32 = (s_2_3.value() as u32);
        // D s_2_5: read-var d:i64
        let s_2_5: i64 = fn_state.d;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: call R_set(s_2_6, s_2_4)
        let s_2_7: () = R_set(state, tracer, s_2_6, s_2_4);
        // N s_2_8: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var t:i64
        let s_3_0: i64 = fn_state.t;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #4s : i
        let s_3_3: i128 = 4;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_5: read-var address:u32
        let s_3_5: u32 = fn_state.address;
        // D s_3_6: call MemA_set(s_3_5, s_3_3, s_3_4)
        let s_3_6: () = MemA_set(state, tracer, s_3_5, s_3_3, s_3_4);
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #32s : i
        let s_4_0: i128 = 32;
        // C s_4_1: const #0u : u8
        let s_4_1: bool = false;
        // C s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 1u16);
        // D s_4_3: bits-cast zx s_4_2 -> bv length s_4_0
        let s_4_3: Bits = s_4_2.zero_extend(s_4_0);
        // D s_4_4: cast reint s_4_3 -> u32
        let s_4_4: u32 = (s_4_3.value() as u32);
        // D s_4_5: read-var d:i64
        let s_4_5: i64 = fn_state.d;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call R_set(s_4_6, s_4_4)
        let s_4_7: () = R_set(state, tracer, s_4_6, s_4_4);
        // N s_4_8: return
        return;
    }
}
