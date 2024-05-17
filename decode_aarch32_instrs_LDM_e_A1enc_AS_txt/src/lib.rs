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
use ConditionPassed::*;
use execute_aarch32_instrs_LDM_e_Op_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_LDM_e_A1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
    register_list: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        wordhigher: bool,
        increment_name: bool,
        wback: bool,
        gs_323260: bool,
        registers: u16,
        n: i64,
        cond: u8,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
        register_list: u16,
    }
    let fn_state = FunctionState {
        cond,
        P,
        U,
        W,
        Rn,
        register_list,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var Rn:u8
        let s_2_6: u8 = fn_state.Rn;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var n <= s_2_9
        fn_state.n = s_2_9;
        // D s_2_11: read-var register_list:u15
        let s_2_11: u16 = fn_state.register_list;
        // D s_2_12: write-var registers <= s_2_11
        fn_state.registers = s_2_11;
        // D s_2_13: read-var W:u8
        let s_2_13: bool = fn_state.W;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // C s_2_15: const #1u : u8
        let s_2_15: bool = true;
        // C s_2_16: cast zx s_2_15 -> bv
        let s_2_16: Bits = Bits::new(s_2_15 as u128, 1u16);
        // D s_2_17: cmp-eq s_2_14 s_2_16
        let s_2_17: bool = ((s_2_14) == (s_2_16));
        // D s_2_18: write-var wback <= s_2_17
        fn_state.wback = s_2_17;
        // D s_2_19: read-var U:u8
        let s_2_19: bool = fn_state.U;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // C s_2_21: const #1u : u8
        let s_2_21: bool = true;
        // C s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 1u16);
        // D s_2_23: cmp-eq s_2_20 s_2_22
        let s_2_23: bool = ((s_2_20) == (s_2_22));
        // D s_2_24: write-var increment_name <= s_2_23
        fn_state.increment_name = s_2_23;
        // D s_2_25: read-var P:u8
        let s_2_25: bool = fn_state.P;
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 1u16);
        // D s_2_27: read-var U:u8
        let s_2_27: bool = fn_state.U;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 1u16);
        // D s_2_29: cmp-eq s_2_26 s_2_28
        let s_2_29: bool = ((s_2_26) == (s_2_28));
        // D s_2_30: write-var wordhigher <= s_2_29
        fn_state.wordhigher = s_2_29;
        // C s_2_31: const #15s : i
        let s_2_31: i128 = 15;
        // D s_2_32: read-var n:i64
        let s_2_32: i64 = fn_state.n;
        // D s_2_33: cast zx s_2_32 -> i
        let s_2_33: i128 = (i128::try_from(s_2_32).unwrap());
        // D s_2_34: cmp-eq s_2_33 s_2_31
        let s_2_34: bool = ((s_2_33) == (s_2_31));
        // N s_2_35: branch s_2_34 b9 b3
        if s_2_34 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var wback:u8
        let s_3_0: bool = fn_state.wback;
        // N s_3_1: branch s_3_0 b8 b4
        if s_3_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#323260 <= s_4_0
        fn_state.gs_323260 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#323260:u8
        let s_5_0: bool = fn_state.gs_323260;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var increment_name:u8
        let s_6_0: bool = fn_state.increment_name;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: read-var registers:u15
        let s_6_2: u16 = fn_state.registers;
        // D s_6_3: read-var wback:u8
        let s_6_3: bool = fn_state.wback;
        // D s_6_4: read-var wordhigher:u8
        let s_6_4: bool = fn_state.wordhigher;
        // D s_6_5: call execute_aarch32_instrs_LDM_e_Op_AS_txt(s_6_0, s_6_1, s_6_2, s_6_3, s_6_4)
        let s_6_5: () = execute_aarch32_instrs_LDM_e_Op_AS_txt(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
        );
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var registers:u15
        let s_8_0: u16 = fn_state.registers;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 15u16);
        // D s_8_2: read-var n:i64
        let s_8_2: i64 = fn_state.n;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // C s_8_4: const #1u : u64
        let s_8_4: u64 = 1;
        // D s_8_5: bit-extract s_8_1 s_8_3 s_8_4
        let s_8_5: Bits = (Bits::new(
            ((s_8_1) >> (s_8_3)).value(),
            u16::try_from(s_8_4).unwrap(),
        ));
        // D s_8_6: cast reint s_8_5 -> u8
        let s_8_6: bool = ((s_8_5.value()) != 0);
        // C s_8_7: const #0s : i
        let s_8_7: i128 = 0;
        // C s_8_8: const #0u : u64
        let s_8_8: u64 = 0;
        // D s_8_9: cast zx s_8_6 -> u64
        let s_8_9: u64 = (s_8_6 as u64);
        // C s_8_10: const #1u : u64
        let s_8_10: u64 = 1;
        // D s_8_11: and s_8_9 s_8_10
        let s_8_11: u64 = ((s_8_9) & (s_8_10));
        // D s_8_12: cmp-eq s_8_11 s_8_10
        let s_8_12: bool = ((s_8_11) == (s_8_10));
        // D s_8_13: lsl s_8_9 s_8_7
        let s_8_13: u64 = s_8_9 << s_8_7;
        // D s_8_14: or s_8_8 s_8_13
        let s_8_14: u64 = ((s_8_8) | (s_8_13));
        // D s_8_15: cmpl s_8_13
        let s_8_15: u64 = !s_8_13;
        // D s_8_16: and s_8_8 s_8_15
        let s_8_16: u64 = ((s_8_8) & (s_8_15));
        // D s_8_17: select s_8_12 s_8_14 s_8_16
        let s_8_17: u64 = if s_8_12 { s_8_14 } else { s_8_16 };
        // D s_8_18: cast trunc s_8_17 -> u8
        let s_8_18: bool = ((s_8_17) != 0);
        // D s_8_19: cast zx s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 1u16);
        // C s_8_20: const #1u : u8
        let s_8_20: bool = true;
        // C s_8_21: cast zx s_8_20 -> bv
        let s_8_21: Bits = Bits::new(s_8_20 as u128, 1u16);
        // D s_8_22: cmp-eq s_8_19 s_8_21
        let s_8_22: bool = ((s_8_19) == (s_8_21));
        // D s_8_23: write-var gs#323260 <= s_8_22
        fn_state.gs_323260 = s_8_22;
        // N s_8_24: jump b5
        return block_5(state, tracer, fn_state);
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
}
