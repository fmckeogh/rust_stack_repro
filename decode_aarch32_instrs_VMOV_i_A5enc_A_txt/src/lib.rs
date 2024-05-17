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
use execute_aarch32_instrs_VMOV_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_i_A5enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    D: bool,
    imm3: u8,
    Vd: u8,
    cmode: u8,
    Q: bool,
    op: bool,
    imm4: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        gs_312929: bool,
        imm64: u64,
        d: i64,
        gs_312926: bool,
        gs_312930: bool,
        ga_356164: i64,
        gs_312933: bool,
        imm32shadow_7541: u32,
        i: bool,
        D: bool,
        imm3: u8,
        Vd: u8,
        cmode: u8,
        Q: bool,
        op: bool,
        imm4: u8,
    }
    let fn_state = FunctionState {
        i,
        D,
        imm3,
        Vd,
        cmode,
        Q,
        op,
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
        // D s_2_0: read-var op:u8
        let s_2_0: bool = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b24 b3
        if s_2_4 {
            return block_24(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#312926 <= s_3_0
        fn_state.gs_312926 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#312926:u8
        let s_4_0: bool = fn_state.gs_312926;
        // N s_4_1: branch s_4_0 b23 b5
        if s_4_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#312929 <= s_5_0
        fn_state.gs_312929 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#312929:u8
        let s_6_0: bool = fn_state.gs_312929;
        // N s_6_1: branch s_6_0 b22 b7
        if s_6_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var op:u8
        let s_7_0: bool = fn_state.op;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b21 b8
        if s_7_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#312930 <= s_8_0
        fn_state.gs_312930 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#312930:u8
        let s_9_0: bool = fn_state.gs_312930;
        // N s_9_1: branch s_9_0 b20 b10
        if s_9_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var Q:u8
        let s_10_0: bool = fn_state.Q;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b19 b11
        if s_10_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#312933 <= s_11_0
        fn_state.gs_312933 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#312933:u8
        let s_12_0: bool = fn_state.gs_312933;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var imm32:u32
        let s_13_0: u32 = fn_state.imm32;
        // D s_13_1: write-var imm32shadow#7541 <= s_13_0
        fn_state.imm32shadow_7541 = s_13_0;
        // D s_13_2: read-var i:u8
        let s_13_2: bool = fn_state.i;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: read-var imm3:u8
        let s_13_4: u8 = fn_state.imm3;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 3u16);
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: cast reint s_13_5 -> u128
        let s_13_8: u128 = (s_13_5.value() as u128);
        // D s_13_9: size-of s_13_5
        let s_13_9: u16 = s_13_5.length();
        // D s_13_10: lsl s_13_6 s_13_9
        let s_13_10: u128 = s_13_6 << s_13_9;
        // D s_13_11: or s_13_10 s_13_8
        let s_13_11: u128 = ((s_13_10) | (s_13_8));
        // D s_13_12: add s_13_7 s_13_9
        let s_13_12: u16 = (s_13_7 + s_13_9);
        // D s_13_13: create-bits s_13_11 s_13_12
        let s_13_13: Bits = Bits::new(s_13_11, s_13_12);
        // D s_13_14: cast reint s_13_13 -> u8
        let s_13_14: u8 = (s_13_13.value() as u8);
        // D s_13_15: cast zx s_13_14 -> bv
        let s_13_15: Bits = Bits::new(s_13_14 as u128, 4u16);
        // D s_13_16: read-var imm4:u8
        let s_13_16: u8 = fn_state.imm4;
        // D s_13_17: cast zx s_13_16 -> bv
        let s_13_17: Bits = Bits::new(s_13_16 as u128, 4u16);
        // D s_13_18: cast reint s_13_15 -> u128
        let s_13_18: u128 = (s_13_15.value() as u128);
        // D s_13_19: size-of s_13_15
        let s_13_19: u16 = s_13_15.length();
        // D s_13_20: cast reint s_13_17 -> u128
        let s_13_20: u128 = (s_13_17.value() as u128);
        // D s_13_21: size-of s_13_17
        let s_13_21: u16 = s_13_17.length();
        // D s_13_22: lsl s_13_18 s_13_21
        let s_13_22: u128 = s_13_18 << s_13_21;
        // D s_13_23: or s_13_22 s_13_20
        let s_13_23: u128 = ((s_13_22) | (s_13_20));
        // D s_13_24: add s_13_19 s_13_21
        let s_13_24: u16 = (s_13_19 + s_13_21);
        // D s_13_25: create-bits s_13_23 s_13_24
        let s_13_25: Bits = Bits::new(s_13_23, s_13_24);
        // D s_13_26: cast reint s_13_25 -> u8
        let s_13_26: u8 = (s_13_25.value() as u8);
        // D s_13_27: read-var op:u8
        let s_13_27: bool = fn_state.op;
        // D s_13_28: read-var cmode:u8
        let s_13_28: u8 = fn_state.cmode;
        // D s_13_29: call AdvSIMDExpandImm(s_13_27, s_13_28, s_13_26)
        let s_13_29: u64 = AdvSIMDExpandImm(state, tracer, s_13_27, s_13_28, s_13_26);
        // D s_13_30: write-var imm64 <= s_13_29
        fn_state.imm64 = s_13_29;
        // N s_13_31: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var D:u8
        let s_14_0: bool = fn_state.D;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // D s_14_2: read-var Vd:u8
        let s_14_2: u8 = fn_state.Vd;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 4u16);
        // D s_14_4: cast reint s_14_1 -> u128
        let s_14_4: u128 = (s_14_1.value() as u128);
        // D s_14_5: size-of s_14_1
        let s_14_5: u16 = s_14_1.length();
        // D s_14_6: cast reint s_14_3 -> u128
        let s_14_6: u128 = (s_14_3.value() as u128);
        // D s_14_7: size-of s_14_3
        let s_14_7: u16 = s_14_3.length();
        // D s_14_8: lsl s_14_4 s_14_7
        let s_14_8: u128 = s_14_4 << s_14_7;
        // D s_14_9: or s_14_8 s_14_6
        let s_14_9: u128 = ((s_14_8) | (s_14_6));
        // D s_14_10: add s_14_5 s_14_7
        let s_14_10: u16 = (s_14_5 + s_14_7);
        // D s_14_11: create-bits s_14_9 s_14_10
        let s_14_11: Bits = Bits::new(s_14_9, s_14_10);
        // D s_14_12: cast reint s_14_11 -> u8
        let s_14_12: u8 = (s_14_11.value() as u8);
        // D s_14_13: cast zx s_14_12 -> bv
        let s_14_13: Bits = Bits::new(s_14_12 as u128, 5u16);
        // D s_14_14: cast zx s_14_13 -> i
        let s_14_14: i128 = (s_14_13.value() as i128);
        // D s_14_15: cast reint s_14_14 -> i64
        let s_14_15: i64 = (s_14_14 as i64);
        // D s_14_16: write-var d <= s_14_15
        fn_state.d = s_14_15;
        // D s_14_17: read-var Q:u8
        let s_14_17: bool = fn_state.Q;
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 1u16);
        // C s_14_19: const #0u : u8
        let s_14_19: bool = false;
        // C s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 1u16);
        // D s_14_21: cmp-eq s_14_18 s_14_20
        let s_14_21: bool = ((s_14_18) == (s_14_20));
        // N s_14_22: branch s_14_21 b17 b15
        if s_14_21 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2s : i64
        let s_15_0: i64 = 2;
        // D s_15_1: write-var ga#356164 <= s_15_0
        fn_state.ga_356164 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#356164:i64
        let s_16_0: i64 = fn_state.ga_356164;
        // C s_16_1: const #1u : u8
        let s_16_1: bool = true;
        // D s_16_2: read-var d:i64
        let s_16_2: i64 = fn_state.d;
        // D s_16_3: read-var imm32shadow#7541:u32
        let s_16_3: u32 = fn_state.imm32shadow_7541;
        // D s_16_4: read-var imm64:u64
        let s_16_4: u64 = fn_state.imm64;
        // C s_16_5: const #0u : u8
        let s_16_5: bool = false;
        // D s_16_6: call execute_aarch32_instrs_VMOV_i_Op_A_txt(s_16_1, s_16_2, s_16_3, s_16_4, s_16_0, s_16_5)
        let s_16_6: () = execute_aarch32_instrs_VMOV_i_Op_A_txt(
            state,
            tracer,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
            s_16_0,
            s_16_5,
        );
        // N s_16_7: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i64
        let s_17_0: i64 = 1;
        // D s_17_1: write-var ga#356164 <= s_17_0
        fn_state.ga_356164 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0s : i
        let s_19_0: i128 = 0;
        // D s_19_1: read-var Vd:u8
        let s_19_1: u8 = fn_state.Vd;
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 4u16);
        // C s_19_3: const #1u : u64
        let s_19_3: u64 = 1;
        // D s_19_4: bit-extract s_19_2 s_19_0 s_19_3
        let s_19_4: Bits = (Bits::new(
            ((s_19_2) >> (s_19_0)).value(),
            u16::try_from(s_19_3).unwrap(),
        ));
        // D s_19_5: cast reint s_19_4 -> u8
        let s_19_5: bool = ((s_19_4.value()) != 0);
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // C s_19_7: const #0u : u64
        let s_19_7: u64 = 0;
        // D s_19_8: cast zx s_19_5 -> u64
        let s_19_8: u64 = (s_19_5 as u64);
        // C s_19_9: const #1u : u64
        let s_19_9: u64 = 1;
        // D s_19_10: and s_19_8 s_19_9
        let s_19_10: u64 = ((s_19_8) & (s_19_9));
        // D s_19_11: cmp-eq s_19_10 s_19_9
        let s_19_11: bool = ((s_19_10) == (s_19_9));
        // D s_19_12: lsl s_19_8 s_19_6
        let s_19_12: u64 = s_19_8 << s_19_6;
        // D s_19_13: or s_19_7 s_19_12
        let s_19_13: u64 = ((s_19_7) | (s_19_12));
        // D s_19_14: cmpl s_19_12
        let s_19_14: u64 = !s_19_12;
        // D s_19_15: and s_19_7 s_19_14
        let s_19_15: u64 = ((s_19_7) & (s_19_14));
        // D s_19_16: select s_19_11 s_19_13 s_19_15
        let s_19_16: u64 = if s_19_11 { s_19_13 } else { s_19_15 };
        // D s_19_17: cast trunc s_19_16 -> u8
        let s_19_17: bool = ((s_19_16) != 0);
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 1u16);
        // C s_19_19: const #1u : u8
        let s_19_19: bool = true;
        // C s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 1u16);
        // D s_19_21: cmp-eq s_19_18 s_19_20
        let s_19_21: bool = ((s_19_18) == (s_19_20));
        // D s_19_22: write-var gs#312933 <= s_19_21
        fn_state.gs_312933 = s_19_21;
        // N s_19_23: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var cmode:u8
        let s_21_0: u8 = fn_state.cmode;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // C s_21_2: const #14u : u8
        let s_21_2: u8 = 14;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 4u16);
        // D s_21_4: cmp-ne s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) != (s_21_3));
        // D s_21_5: write-var gs#312930 <= s_21_4
        fn_state.gs_312930 = s_21_4;
        // N s_21_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #2s : i
        let s_23_0: i128 = 2;
        // D s_23_1: read-var cmode:u8
        let s_23_1: u8 = fn_state.cmode;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 4u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_0 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 2u16);
        // C s_23_10: const #3u : u8
        let s_23_10: u8 = 3;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 2u16);
        // D s_23_12: cmp-ne s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) != (s_23_11));
        // D s_23_13: write-var gs#312929 <= s_23_12
        fn_state.gs_312929 = s_23_12;
        // N s_23_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var cmode:u8
        let s_24_1: u8 = fn_state.cmode;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 4u16);
        // C s_24_3: const #1u : u64
        let s_24_3: u64 = 1;
        // D s_24_4: bit-extract s_24_2 s_24_0 s_24_3
        let s_24_4: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_5: cast reint s_24_4 -> u8
        let s_24_5: bool = ((s_24_4.value()) != 0);
        // C s_24_6: const #0s : i
        let s_24_6: i128 = 0;
        // C s_24_7: const #0u : u64
        let s_24_7: u64 = 0;
        // D s_24_8: cast zx s_24_5 -> u64
        let s_24_8: u64 = (s_24_5 as u64);
        // C s_24_9: const #1u : u64
        let s_24_9: u64 = 1;
        // D s_24_10: and s_24_8 s_24_9
        let s_24_10: u64 = ((s_24_8) & (s_24_9));
        // D s_24_11: cmp-eq s_24_10 s_24_9
        let s_24_11: bool = ((s_24_10) == (s_24_9));
        // D s_24_12: lsl s_24_8 s_24_6
        let s_24_12: u64 = s_24_8 << s_24_6;
        // D s_24_13: or s_24_7 s_24_12
        let s_24_13: u64 = ((s_24_7) | (s_24_12));
        // D s_24_14: cmpl s_24_12
        let s_24_14: u64 = !s_24_12;
        // D s_24_15: and s_24_7 s_24_14
        let s_24_15: u64 = ((s_24_7) & (s_24_14));
        // D s_24_16: select s_24_11 s_24_13 s_24_15
        let s_24_16: u64 = if s_24_11 { s_24_13 } else { s_24_15 };
        // D s_24_17: cast trunc s_24_16 -> u8
        let s_24_17: bool = ((s_24_16) != 0);
        // D s_24_18: cast zx s_24_17 -> bv
        let s_24_18: Bits = Bits::new(s_24_17 as u128, 1u16);
        // C s_24_19: const #1u : u8
        let s_24_19: bool = true;
        // C s_24_20: cast zx s_24_19 -> bv
        let s_24_20: Bits = Bits::new(s_24_19 as u128, 1u16);
        // D s_24_21: cmp-eq s_24_18 s_24_20
        let s_24_21: bool = ((s_24_18) == (s_24_20));
        // D s_24_22: write-var gs#312926 <= s_24_21
        fn_state.gs_312926 = s_24_21;
        // N s_24_23: jump b4
        return block_4(state, tracer, fn_state);
    }
}
