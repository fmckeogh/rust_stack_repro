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
use AdvSIMDExpandImm::*;
use execute_aarch32_instrs_VMVN_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMVN_i_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    D: bool,
    imm3: u8,
    Vd: u8,
    cmode: u8,
    Q: bool,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_314190: bool,
        imm64: u64,
        d: i64,
        gs_314187: bool,
        gs_314184: bool,
        ga_357169: i64,
        i: bool,
        D: bool,
        imm3: u8,
        Vd: u8,
        cmode: u8,
        Q: bool,
        imm4: u8,
    }
    let fn_state = FunctionState {
        i,
        D,
        imm3,
        Vd,
        cmode,
        Q,
        imm4,
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
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var cmode:u8
        let s_2_1: u8 = fn_state.cmode;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b19 b3
        if s_2_21 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#314184 <= s_3_0
        fn_state.gs_314184 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#314184:u8
        let s_4_0: bool = fn_state.gs_314184;
        // N s_4_1: branch s_4_0 b18 b5
        if s_4_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var cmode:u8
        let s_5_1: u8 = fn_state.cmode;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 4u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #2s : i
        let s_5_5: i128 = 2;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_0 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 3u16);
        // C s_5_10: const #7u : u8
        let s_5_10: u8 = 7;
        // C s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 3u16);
        // D s_5_12: cmp-eq s_5_9 s_5_11
        let s_5_12: bool = ((s_5_9) == (s_5_11));
        // D s_5_13: write-var gs#314187 <= s_5_12
        fn_state.gs_314187 = s_5_12;
        // N s_5_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#314187:u8
        let s_6_0: bool = fn_state.gs_314187;
        // N s_6_1: branch s_6_0 b17 b7
        if s_6_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var Q:u8
        let s_7_0: bool = fn_state.Q;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b16 b8
        if s_7_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#314190 <= s_8_0
        fn_state.gs_314190 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#314190:u8
        let s_9_0: bool = fn_state.gs_314190;
        // N s_9_1: branch s_9_0 b15 b10
        if s_9_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var i:u8
        let s_10_0: bool = fn_state.i;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: read-var imm3:u8
        let s_10_2: u8 = fn_state.imm3;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 3u16);
        // D s_10_4: cast reint s_10_1 -> u128
        let s_10_4: u128 = (s_10_1.value() as u128);
        // D s_10_5: size-of s_10_1
        let s_10_5: u16 = s_10_1.length();
        // D s_10_6: cast reint s_10_3 -> u128
        let s_10_6: u128 = (s_10_3.value() as u128);
        // D s_10_7: size-of s_10_3
        let s_10_7: u16 = s_10_3.length();
        // D s_10_8: lsl s_10_4 s_10_7
        let s_10_8: u128 = s_10_4 << s_10_7;
        // D s_10_9: or s_10_8 s_10_6
        let s_10_9: u128 = ((s_10_8) | (s_10_6));
        // D s_10_10: add s_10_5 s_10_7
        let s_10_10: u16 = (s_10_5 + s_10_7);
        // D s_10_11: create-bits s_10_9 s_10_10
        let s_10_11: Bits = Bits::new(s_10_9, s_10_10);
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: u8 = (s_10_11.value() as u8);
        // D s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 4u16);
        // D s_10_14: read-var imm4:u8
        let s_10_14: u8 = fn_state.imm4;
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 4u16);
        // D s_10_16: cast reint s_10_13 -> u128
        let s_10_16: u128 = (s_10_13.value() as u128);
        // D s_10_17: size-of s_10_13
        let s_10_17: u16 = s_10_13.length();
        // D s_10_18: cast reint s_10_15 -> u128
        let s_10_18: u128 = (s_10_15.value() as u128);
        // D s_10_19: size-of s_10_15
        let s_10_19: u16 = s_10_15.length();
        // D s_10_20: lsl s_10_16 s_10_19
        let s_10_20: u128 = s_10_16 << s_10_19;
        // D s_10_21: or s_10_20 s_10_18
        let s_10_21: u128 = ((s_10_20) | (s_10_18));
        // D s_10_22: add s_10_17 s_10_19
        let s_10_22: u16 = (s_10_17 + s_10_19);
        // D s_10_23: create-bits s_10_21 s_10_22
        let s_10_23: Bits = Bits::new(s_10_21, s_10_22);
        // D s_10_24: cast reint s_10_23 -> u8
        let s_10_24: u8 = (s_10_23.value() as u8);
        // C s_10_25: const #1u : u8
        let s_10_25: bool = true;
        // D s_10_26: read-var cmode:u8
        let s_10_26: u8 = fn_state.cmode;
        // D s_10_27: call AdvSIMDExpandImm(s_10_25, s_10_26, s_10_24)
        let s_10_27: u64 = AdvSIMDExpandImm(state, tracer, s_10_25, s_10_26, s_10_24);
        // D s_10_28: write-var imm64 <= s_10_27
        fn_state.imm64 = s_10_27;
        // N s_10_29: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var D:u8
        let s_11_0: bool = fn_state.D;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // D s_11_2: read-var Vd:u8
        let s_11_2: u8 = fn_state.Vd;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 4u16);
        // D s_11_4: cast reint s_11_1 -> u128
        let s_11_4: u128 = (s_11_1.value() as u128);
        // D s_11_5: size-of s_11_1
        let s_11_5: u16 = s_11_1.length();
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: lsl s_11_4 s_11_7
        let s_11_8: u128 = s_11_4 << s_11_7;
        // D s_11_9: or s_11_8 s_11_6
        let s_11_9: u128 = ((s_11_8) | (s_11_6));
        // D s_11_10: add s_11_5 s_11_7
        let s_11_10: u16 = (s_11_5 + s_11_7);
        // D s_11_11: create-bits s_11_9 s_11_10
        let s_11_11: Bits = Bits::new(s_11_9, s_11_10);
        // D s_11_12: cast reint s_11_11 -> u8
        let s_11_12: u8 = (s_11_11.value() as u8);
        // D s_11_13: cast zx s_11_12 -> bv
        let s_11_13: Bits = Bits::new(s_11_12 as u128, 5u16);
        // D s_11_14: cast zx s_11_13 -> i
        let s_11_14: i128 = (s_11_13.value() as i128);
        // D s_11_15: cast reint s_11_14 -> i64
        let s_11_15: i64 = (s_11_14 as i64);
        // D s_11_16: write-var d <= s_11_15
        fn_state.d = s_11_15;
        // D s_11_17: read-var Q:u8
        let s_11_17: bool = fn_state.Q;
        // D s_11_18: cast zx s_11_17 -> bv
        let s_11_18: Bits = Bits::new(s_11_17 as u128, 1u16);
        // C s_11_19: const #0u : u8
        let s_11_19: bool = false;
        // C s_11_20: cast zx s_11_19 -> bv
        let s_11_20: Bits = Bits::new(s_11_19 as u128, 1u16);
        // D s_11_21: cmp-eq s_11_18 s_11_20
        let s_11_21: bool = ((s_11_18) == (s_11_20));
        // N s_11_22: branch s_11_21 b14 b12
        if s_11_21 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2s : i64
        let s_12_0: i64 = 2;
        // D s_12_1: write-var ga#357169 <= s_12_0
        fn_state.ga_357169 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#357169:i64
        let s_13_0: i64 = fn_state.ga_357169;
        // D s_13_1: read-var d:i64
        let s_13_1: i64 = fn_state.d;
        // D s_13_2: read-var imm64:u64
        let s_13_2: u64 = fn_state.imm64;
        // D s_13_3: call execute_aarch32_instrs_VMVN_i_Op_A_txt(s_13_1, s_13_2, s_13_0)
        let s_13_3: () = execute_aarch32_instrs_VMVN_i_Op_A_txt(
            state,
            tracer,
            s_13_1,
            s_13_2,
            s_13_0,
        );
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i64
        let s_14_0: i64 = 1;
        // D s_14_1: write-var ga#357169 <= s_14_0
        fn_state.ga_357169 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
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
        // C s_16_0: const #0s : i
        let s_16_0: i128 = 0;
        // D s_16_1: read-var Vd:u8
        let s_16_1: u8 = fn_state.Vd;
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 4u16);
        // C s_16_3: const #1u : u64
        let s_16_3: u64 = 1;
        // D s_16_4: bit-extract s_16_2 s_16_0 s_16_3
        let s_16_4: Bits = (Bits::new(
            ((s_16_2) >> (s_16_0)).value(),
            u16::try_from(s_16_3).unwrap(),
        ));
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: bool = ((s_16_4.value()) != 0);
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // C s_16_7: const #0u : u64
        let s_16_7: u64 = 0;
        // D s_16_8: cast zx s_16_5 -> u64
        let s_16_8: u64 = (s_16_5 as u64);
        // C s_16_9: const #1u : u64
        let s_16_9: u64 = 1;
        // D s_16_10: and s_16_8 s_16_9
        let s_16_10: u64 = ((s_16_8) & (s_16_9));
        // D s_16_11: cmp-eq s_16_10 s_16_9
        let s_16_11: bool = ((s_16_10) == (s_16_9));
        // D s_16_12: lsl s_16_8 s_16_6
        let s_16_12: u64 = s_16_8 << s_16_6;
        // D s_16_13: or s_16_7 s_16_12
        let s_16_13: u64 = ((s_16_7) | (s_16_12));
        // D s_16_14: cmpl s_16_12
        let s_16_14: u64 = !s_16_12;
        // D s_16_15: and s_16_7 s_16_14
        let s_16_15: u64 = ((s_16_7) & (s_16_14));
        // D s_16_16: select s_16_11 s_16_13 s_16_15
        let s_16_16: u64 = if s_16_11 { s_16_13 } else { s_16_15 };
        // D s_16_17: cast trunc s_16_16 -> u8
        let s_16_17: bool = ((s_16_16) != 0);
        // D s_16_18: cast zx s_16_17 -> bv
        let s_16_18: Bits = Bits::new(s_16_17 as u128, 1u16);
        // C s_16_19: const #1u : u8
        let s_16_19: bool = true;
        // C s_16_20: cast zx s_16_19 -> bv
        let s_16_20: Bits = Bits::new(s_16_19 as u128, 1u16);
        // D s_16_21: cmp-eq s_16_18 s_16_20
        let s_16_21: bool = ((s_16_18) == (s_16_20));
        // D s_16_22: write-var gs#314190 <= s_16_21
        fn_state.gs_314190 = s_16_21;
        // N s_16_23: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#314187 <= s_18_0
        fn_state.gs_314187 = s_18_0;
        // N s_18_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #2s : i
        let s_19_0: i128 = 2;
        // D s_19_1: read-var cmode:u8
        let s_19_1: u8 = fn_state.cmode;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1s : i64
        let s_19_3: i64 = 1;
        // C s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // C s_19_5: const #1s : i
        let s_19_5: i128 = 1;
        // C s_19_6: add s_19_5 s_19_4
        let s_19_6: i128 = (s_19_5 + s_19_4);
        // D s_19_7: bit-extract s_19_2 s_19_0 s_19_6
        let s_19_7: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_6).unwrap(),
        ));
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: u8 = (s_19_7.value() as u8);
        // D s_19_9: cast zx s_19_8 -> bv
        let s_19_9: Bits = Bits::new(s_19_8 as u128, 2u16);
        // C s_19_10: const #3u : u8
        let s_19_10: u8 = 3;
        // C s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 2u16);
        // D s_19_12: cmp-ne s_19_9 s_19_11
        let s_19_12: bool = ((s_19_9) != (s_19_11));
        // D s_19_13: write-var gs#314184 <= s_19_12
        fn_state.gs_314184 = s_19_12;
        // N s_19_14: jump b4
        return block_4(state, tracer, fn_state);
    }
}
