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
use integer_subrange::*;
use AArch32_STC::*;
use common::*;
pub fn AArch32_SysRegRead<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cp_num: i128,
    instr: u32,
    address: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        CRd: u8,
        gs_141911: bool,
        cp_num: i128,
        instr: u32,
        address: u32,
    }
    let fn_state = FunctionState {
        cp_num,
        instr,
        address,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #12s : i
        let s_0_0: i128 = 12;
        // D s_0_1: read-var instr:u32
        let s_0_1: u32 = fn_state.instr;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 32u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #3s : i
        let s_0_5: i128 = 3;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: write-var CRd <= s_0_8
        fn_state.CRd = s_0_8;
        // D s_0_10: read-var CRd:u8
        let s_0_10: u8 = fn_state.CRd;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #5u : u8
        let s_0_12: u8 = 5;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b5 b1
        if s_0_14 {
            return block_5(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#141911 <= s_1_0
        fn_state.gs_141911 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#141911:u8
        let s_2_0: bool = fn_state.gs_141911;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #3s : i
        let s_4_0: i128 = 3;
        // C s_4_1: const #0s : i
        let s_4_1: i128 = 0;
        // D s_4_2: read-var cp_num:i
        let s_4_2: i128 = fn_state.cp_num;
        // D s_4_3: call integer_subrange(s_4_2, s_4_0, s_4_1)
        let s_4_3: Bits = integer_subrange(state, tracer, s_4_2, s_4_0, s_4_1);
        // D s_4_4: cast reint s_4_3 -> u8
        let s_4_4: u8 = (s_4_3.value() as u8);
        // D s_4_5: read-var CRd:u8
        let s_4_5: u8 = fn_state.CRd;
        // D s_4_6: read-var address:u32
        let s_4_6: u32 = fn_state.address;
        // D s_4_7: call AArch32_STC(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = AArch32_STC(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #14u : u8
        let s_5_0: u8 = 14;
        // C s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // C s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var cp_num:i
        let s_5_5: i128 = fn_state.cp_num;
        // D s_5_6: cmp-eq s_5_5 s_5_4
        let s_5_6: bool = ((s_5_5) == (s_5_4));
        // D s_5_7: write-var gs#141911 <= s_5_6
        fn_state.gs_141911 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
