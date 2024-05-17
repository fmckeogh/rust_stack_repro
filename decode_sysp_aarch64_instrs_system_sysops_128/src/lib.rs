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
use AArch64_CheckSystemAccess::*;
use u__id::*;
use HaveSysInstr128::*;
use execute_aarch64_instrs_system_sysops_128::*;
use common::*;
pub fn decode_sysp_aarch64_instrs_system_sysops_128<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    op2: u8,
    CRm: u8,
    CRn: u8,
    op1: u8,
    L: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_173857: bool,
        t: i64,
        ga_269854: i64,
        Rt: u8,
        op2: u8,
        CRm: u8,
        CRn: u8,
        op1: u8,
        L: bool,
    }
    let fn_state = FunctionState {
        Rt,
        op2,
        CRm,
        CRn,
        op1,
        L,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveSysInstr128(s_0_0)
        let s_0_1: bool = HaveSysInstr128(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b11 b1
        if s_0_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var Rt:u8
        let s_1_1: u8 = fn_state.Rt;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b10 b2
        if s_1_21 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#173857 <= s_2_0
        fn_state.gs_173857 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#173857:u8
        let s_3_0: bool = fn_state.gs_173857;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: u8 = 1;
        // D s_4_1: read-var op1:u8
        let s_4_1: u8 = fn_state.op1;
        // D s_4_2: read-var CRn:u8
        let s_4_2: u8 = fn_state.CRn;
        // D s_4_3: read-var CRm:u8
        let s_4_3: u8 = fn_state.CRm;
        // D s_4_4: read-var op2:u8
        let s_4_4: u8 = fn_state.op2;
        // D s_4_5: read-var Rt:u8
        let s_4_5: u8 = fn_state.Rt;
        // D s_4_6: read-var L:u8
        let s_4_6: bool = fn_state.L;
        // D s_4_7: call AArch64_CheckSystemAccess(s_4_0, s_4_1, s_4_2, s_4_3, s_4_4, s_4_5, s_4_6)
        let s_4_7: () = AArch64_CheckSystemAccess(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
        );
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Rt:u8
        let s_5_0: u8 = fn_state.Rt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // C s_5_5: const #31s : i
        let s_5_5: i128 = 31;
        // D s_5_6: read-var t:i64
        let s_5_6: i64 = fn_state.t;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cmp-eq s_5_7 s_5_5
        let s_5_8: bool = ((s_5_7) == (s_5_5));
        // N s_5_9: branch s_5_8 b8 b6
        if s_5_8 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i64
        let s_6_0: i64 = fn_state.t;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #1s : i
        let s_6_4: i128 = 1;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // D s_6_8: write-var ga#269854 <= s_6_7
        fn_state.ga_269854 = s_6_7;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#269854:i64
        let s_7_0: i64 = fn_state.ga_269854;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: read-var op1:u8
        let s_7_2: u8 = fn_state.op1;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: read-var op2:u8
        let s_7_6: u8 = fn_state.op2;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 3u16);
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (s_7_7.value() as i128);
        // D s_7_9: cast reint s_7_8 -> i64
        let s_7_9: i64 = (s_7_8 as i64);
        // D s_7_10: read-var CRn:u8
        let s_7_10: u8 = fn_state.CRn;
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 4u16);
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (s_7_11.value() as i128);
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: read-var CRm:u8
        let s_7_14: u8 = fn_state.CRm;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 4u16);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (s_7_15.value() as i128);
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: read-var t:i64
        let s_7_18: i64 = fn_state.t;
        // D s_7_19: call execute_aarch64_instrs_system_sysops_128(s_7_17, s_7_13, s_7_1, s_7_5, s_7_9, s_7_18, s_7_0)
        let s_7_19: () = execute_aarch64_instrs_system_sysops_128(
            state,
            tracer,
            s_7_17,
            s_7_13,
            s_7_1,
            s_7_5,
            s_7_9,
            s_7_18,
            s_7_0,
        );
        // N s_7_20: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #31s : i64
        let s_8_0: i64 = 31;
        // D s_8_1: write-var ga#269854 <= s_8_0
        fn_state.ga_269854 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var Rt:u8
        let s_10_0: u8 = fn_state.Rt;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 5u16);
        // C s_10_2: const #31u : u8
        let s_10_2: u8 = 31;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 5u16);
        // D s_10_4: cmp-ne s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) != (s_10_3));
        // D s_10_5: write-var gs#173857 <= s_10_4
        fn_state.gs_173857 = s_10_4;
        // N s_10_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
}
