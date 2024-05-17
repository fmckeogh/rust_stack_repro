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
use BitCount::*;
use execute_aarch32_instrs_STMDB_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_STMDB_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    W: bool,
    Rn: u8,
    P: bool,
    M: bool,
    register_list: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        registers: u16,
        n: i64,
        gs_302251: bool,
        wback: bool,
        gs_302253: bool,
        W: bool,
        Rn: u8,
        P: bool,
        M: bool,
        register_list: u16,
    }
    let fn_state = FunctionState {
        W,
        Rn,
        P,
        M,
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
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // D s_2_5: read-var P:u8
        let s_2_5: bool = fn_state.P;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_7: read-var M:u8
        let s_2_7: bool = fn_state.M;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cast reint s_2_6 -> u128
        let s_2_9: u128 = (s_2_6.value() as u128);
        // D s_2_10: size-of s_2_6
        let s_2_10: u16 = s_2_6.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: u8 = (s_2_16.value() as u8);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 2u16);
        // D s_2_19: read-var register_list:u14
        let s_2_19: u16 = fn_state.register_list;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 14u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u16
        let s_2_29: u16 = (s_2_28.value() as u16);
        // D s_2_30: write-var registers <= s_2_29
        fn_state.registers = s_2_29;
        // D s_2_31: read-var W:u8
        let s_2_31: bool = fn_state.W;
        // D s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 1u16);
        // C s_2_33: const #1u : u8
        let s_2_33: bool = true;
        // C s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 1u16);
        // D s_2_35: cmp-eq s_2_32 s_2_34
        let s_2_35: bool = ((s_2_32) == (s_2_34));
        // D s_2_36: write-var wback <= s_2_35
        fn_state.wback = s_2_35;
        // C s_2_37: const #15s : i
        let s_2_37: i128 = 15;
        // D s_2_38: read-var n:i64
        let s_2_38: i64 = fn_state.n;
        // D s_2_39: cast zx s_2_38 -> i
        let s_2_39: i128 = (i128::try_from(s_2_38).unwrap());
        // D s_2_40: cmp-eq s_2_39 s_2_37
        let s_2_40: bool = ((s_2_39) == (s_2_37));
        // N s_2_41: branch s_2_40 b16 b3
        if s_2_40 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var registers:u16
        let s_3_0: u16 = fn_state.registers;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 16u16);
        // D s_3_2: call BitCount(s_3_1)
        let s_3_2: i128 = BitCount(state, tracer, s_3_1);
        // C s_3_3: const #2s : i
        let s_3_3: i128 = 2;
        // D s_3_4: cmp-lt s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) < (s_3_3));
        // D s_3_5: write-var gs#302251 <= s_3_4
        fn_state.gs_302251 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#302251:u8
        let s_4_0: bool = fn_state.gs_302251;
        // N s_4_1: branch s_4_0 b15 b5
        if s_4_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var wback:u8
        let s_5_0: bool = fn_state.wback;
        // N s_5_1: branch s_5_0 b14 b6
        if s_5_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#302253 <= s_6_0
        fn_state.gs_302253 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#302253:u8
        let s_7_0: bool = fn_state.gs_302253;
        // N s_7_1: branch s_7_0 b13 b8
        if s_7_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #13s : i
        let s_8_0: i128 = 13;
        // D s_8_1: read-var registers:u16
        let s_8_1: u16 = fn_state.registers;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 16u16);
        // C s_8_3: const #1u : u64
        let s_8_3: u64 = 1;
        // D s_8_4: bit-extract s_8_2 s_8_0 s_8_3
        let s_8_4: Bits = (Bits::new(
            ((s_8_2) >> (s_8_0)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_5: cast reint s_8_4 -> u8
        let s_8_5: bool = ((s_8_4.value()) != 0);
        // C s_8_6: const #0s : i
        let s_8_6: i128 = 0;
        // C s_8_7: const #0u : u64
        let s_8_7: u64 = 0;
        // D s_8_8: cast zx s_8_5 -> u64
        let s_8_8: u64 = (s_8_5 as u64);
        // C s_8_9: const #1u : u64
        let s_8_9: u64 = 1;
        // D s_8_10: and s_8_8 s_8_9
        let s_8_10: u64 = ((s_8_8) & (s_8_9));
        // D s_8_11: cmp-eq s_8_10 s_8_9
        let s_8_11: bool = ((s_8_10) == (s_8_9));
        // D s_8_12: lsl s_8_8 s_8_6
        let s_8_12: u64 = s_8_8 << s_8_6;
        // D s_8_13: or s_8_7 s_8_12
        let s_8_13: u64 = ((s_8_7) | (s_8_12));
        // D s_8_14: cmpl s_8_12
        let s_8_14: u64 = !s_8_12;
        // D s_8_15: and s_8_7 s_8_14
        let s_8_15: u64 = ((s_8_7) & (s_8_14));
        // D s_8_16: select s_8_11 s_8_13 s_8_15
        let s_8_16: u64 = if s_8_11 { s_8_13 } else { s_8_15 };
        // D s_8_17: cast trunc s_8_16 -> u8
        let s_8_17: bool = ((s_8_16) != 0);
        // D s_8_18: cast zx s_8_17 -> bv
        let s_8_18: Bits = Bits::new(s_8_17 as u128, 1u16);
        // C s_8_19: const #1u : u8
        let s_8_19: bool = true;
        // C s_8_20: cast zx s_8_19 -> bv
        let s_8_20: Bits = Bits::new(s_8_19 as u128, 1u16);
        // D s_8_21: cmp-eq s_8_18 s_8_20
        let s_8_21: bool = ((s_8_18) == (s_8_20));
        // N s_8_22: branch s_8_21 b12 b9
        if s_8_21 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15s : i
        let s_9_0: i128 = 15;
        // D s_9_1: read-var registers:u16
        let s_9_1: u16 = fn_state.registers;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 16u16);
        // C s_9_3: const #1u : u64
        let s_9_3: u64 = 1;
        // D s_9_4: bit-extract s_9_2 s_9_0 s_9_3
        let s_9_4: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_3).unwrap(),
        ));
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: bool = ((s_9_4.value()) != 0);
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // C s_9_7: const #0u : u64
        let s_9_7: u64 = 0;
        // D s_9_8: cast zx s_9_5 -> u64
        let s_9_8: u64 = (s_9_5 as u64);
        // C s_9_9: const #1u : u64
        let s_9_9: u64 = 1;
        // D s_9_10: and s_9_8 s_9_9
        let s_9_10: u64 = ((s_9_8) & (s_9_9));
        // D s_9_11: cmp-eq s_9_10 s_9_9
        let s_9_11: bool = ((s_9_10) == (s_9_9));
        // D s_9_12: lsl s_9_8 s_9_6
        let s_9_12: u64 = s_9_8 << s_9_6;
        // D s_9_13: or s_9_7 s_9_12
        let s_9_13: u64 = ((s_9_7) | (s_9_12));
        // D s_9_14: cmpl s_9_12
        let s_9_14: u64 = !s_9_12;
        // D s_9_15: and s_9_7 s_9_14
        let s_9_15: u64 = ((s_9_7) & (s_9_14));
        // D s_9_16: select s_9_11 s_9_13 s_9_15
        let s_9_16: u64 = if s_9_11 { s_9_13 } else { s_9_15 };
        // D s_9_17: cast trunc s_9_16 -> u8
        let s_9_17: bool = ((s_9_16) != 0);
        // D s_9_18: cast zx s_9_17 -> bv
        let s_9_18: Bits = Bits::new(s_9_17 as u128, 1u16);
        // C s_9_19: const #1u : u8
        let s_9_19: bool = true;
        // C s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // D s_9_21: cmp-eq s_9_18 s_9_20
        let s_9_21: bool = ((s_9_18) == (s_9_20));
        // N s_9_22: branch s_9_21 b11 b10
        if s_9_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: read-var registers:u16
        let s_10_1: u16 = fn_state.registers;
        // D s_10_2: read-var wback:u8
        let s_10_2: bool = fn_state.wback;
        // D s_10_3: call execute_aarch32_instrs_STMDB_Op_A_txt(s_10_0, s_10_1, s_10_2)
        let s_10_3: () = execute_aarch32_instrs_STMDB_Op_A_txt(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
        );
        // N s_10_4: return
        return;
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
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var registers:u16
        let s_14_0: u16 = fn_state.registers;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 16u16);
        // D s_14_2: read-var n:i64
        let s_14_2: i64 = fn_state.n;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // C s_14_4: const #1u : u64
        let s_14_4: u64 = 1;
        // D s_14_5: bit-extract s_14_1 s_14_3 s_14_4
        let s_14_5: Bits = (Bits::new(
            ((s_14_1) >> (s_14_3)).value(),
            u16::try_from(s_14_4).unwrap(),
        ));
        // D s_14_6: cast reint s_14_5 -> u8
        let s_14_6: bool = ((s_14_5.value()) != 0);
        // C s_14_7: const #0s : i
        let s_14_7: i128 = 0;
        // C s_14_8: const #0u : u64
        let s_14_8: u64 = 0;
        // D s_14_9: cast zx s_14_6 -> u64
        let s_14_9: u64 = (s_14_6 as u64);
        // C s_14_10: const #1u : u64
        let s_14_10: u64 = 1;
        // D s_14_11: and s_14_9 s_14_10
        let s_14_11: u64 = ((s_14_9) & (s_14_10));
        // D s_14_12: cmp-eq s_14_11 s_14_10
        let s_14_12: bool = ((s_14_11) == (s_14_10));
        // D s_14_13: lsl s_14_9 s_14_7
        let s_14_13: u64 = s_14_9 << s_14_7;
        // D s_14_14: or s_14_8 s_14_13
        let s_14_14: u64 = ((s_14_8) | (s_14_13));
        // D s_14_15: cmpl s_14_13
        let s_14_15: u64 = !s_14_13;
        // D s_14_16: and s_14_8 s_14_15
        let s_14_16: u64 = ((s_14_8) & (s_14_15));
        // D s_14_17: select s_14_12 s_14_14 s_14_16
        let s_14_17: u64 = if s_14_12 { s_14_14 } else { s_14_16 };
        // D s_14_18: cast trunc s_14_17 -> u8
        let s_14_18: bool = ((s_14_17) != 0);
        // D s_14_19: cast zx s_14_18 -> bv
        let s_14_19: Bits = Bits::new(s_14_18 as u128, 1u16);
        // C s_14_20: const #1u : u8
        let s_14_20: bool = true;
        // C s_14_21: cast zx s_14_20 -> bv
        let s_14_21: Bits = Bits::new(s_14_20 as u128, 1u16);
        // D s_14_22: cmp-eq s_14_19 s_14_21
        let s_14_22: bool = ((s_14_19) == (s_14_21));
        // D s_14_23: write-var gs#302253 <= s_14_22
        fn_state.gs_302253 = s_14_22;
        // N s_14_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#302251 <= s_16_0
        fn_state.gs_302251 = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
