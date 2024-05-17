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
use HaveFP16Ext::*;
use CurrentInstrSet::*;
use ConditionPassed::*;
use place_slice::*;
use execute_aarch32_instrs_VSTR_Op_A_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VSTR_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    Rn: u8,
    Vd: u8,
    size: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_322340: bool,
        imm32: u32,
        esize: i64,
        dshadow_7857: i64,
        gs_322341: bool,
        n: i64,
        d: i64,
        add: bool,
        gs_322365: bool,
        gs_322342: bool,
        U: bool,
        D: bool,
        Rn: u8,
        Vd: u8,
        size: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        Rn,
        Vd,
        size,
        imm8,
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
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #0u : u8
        let s_2_2: u8 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b30 b3
        if s_2_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b29 b4
        if s_3_4 {
            return block_29(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#322340 <= s_4_0
        fn_state.gs_322340 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#322340:u8
        let s_5_0: bool = fn_state.gs_322340;
        // D s_5_1: write-var gs#322341 <= s_5_0
        fn_state.gs_322341 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#322341:u8
        let s_6_0: bool = fn_state.gs_322341;
        // N s_6_1: branch s_6_0 b28 b7
        if s_6_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var size:u8
        let s_7_0: u8 = fn_state.size;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #1u : u8
        let s_7_2: u8 = 1;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 2u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b27 b8
        if s_7_4 {
            return block_27(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#322342 <= s_8_0
        fn_state.gs_322342 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#322342:u8
        let s_9_0: bool = fn_state.gs_322342;
        // N s_9_1: branch s_9_0 b26 b10
        if s_9_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var size:u8
        let s_10_0: u8 = fn_state.size;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (s_10_1.value() as i128);
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #8s : i64
        let s_10_4: i64 = 8;
        // D s_10_5: lsl s_10_4 s_10_3
        let s_10_5: i64 = s_10_4 << s_10_3;
        // D s_10_6: write-var esize <= s_10_5
        fn_state.esize = s_10_5;
        // D s_10_7: read-var U:u8
        let s_10_7: bool = fn_state.U;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 1u16);
        // C s_10_9: const #1u : u8
        let s_10_9: bool = true;
        // C s_10_10: cast zx s_10_9 -> bv
        let s_10_10: Bits = Bits::new(s_10_9 as u128, 1u16);
        // D s_10_11: cmp-eq s_10_8 s_10_10
        let s_10_11: bool = ((s_10_8) == (s_10_10));
        // D s_10_12: write-var add <= s_10_11
        fn_state.add = s_10_11;
        // C s_10_13: const #16s : i
        let s_10_13: i128 = 16;
        // D s_10_14: read-var esize:i64
        let s_10_14: i64 = fn_state.esize;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: cmp-eq s_10_15 s_10_13
        let s_10_16: bool = ((s_10_15) == (s_10_13));
        // N s_10_17: branch s_10_16 b25 b11
        if s_10_16 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #32s : i
        let s_11_0: i128 = 32;
        // C s_11_1: const #0s : i
        let s_11_1: i128 = 0;
        // C s_11_2: const #8s : i
        let s_11_2: i128 = 8;
        // C s_11_3: const #2s : i
        let s_11_3: i128 = 2;
        // D s_11_4: read-var imm8:u8
        let s_11_4: u8 = fn_state.imm8;
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 8u16);
        // D s_11_6: call place_slice(s_11_0, s_11_5, s_11_1, s_11_2, s_11_3)
        let s_11_6: Bits = place_slice(
            state,
            tracer,
            s_11_0,
            s_11_5,
            s_11_1,
            s_11_2,
            s_11_3,
        );
        // D s_11_7: cast reint s_11_6 -> u32
        let s_11_7: u32 = (s_11_6.value() as u32);
        // D s_11_8: write-var imm32 <= s_11_7
        fn_state.imm32 = s_11_7;
        // N s_11_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var size:u8
        let s_12_0: u8 = fn_state.size;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 2u16);
        // C s_12_2: const #1u : u8
        let s_12_2: u8 = 1;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // D s_12_5: not s_12_4
        let s_12_5: bool = !s_12_4;
        // N s_12_6: branch s_12_5 b20 b13
        if s_12_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var Vd:u8
        let s_13_0: u8 = fn_state.Vd;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // D s_13_2: read-var D:u8
        let s_13_2: bool = fn_state.D;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cast reint s_13_1 -> u128
        let s_13_4: u128 = (s_13_1.value() as u128);
        // D s_13_5: size-of s_13_1
        let s_13_5: u16 = s_13_1.length();
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: lsl s_13_4 s_13_7
        let s_13_8: u128 = s_13_4 << s_13_7;
        // D s_13_9: or s_13_8 s_13_6
        let s_13_9: u128 = ((s_13_8) | (s_13_6));
        // D s_13_10: add s_13_5 s_13_7
        let s_13_10: u16 = (s_13_5 + s_13_7);
        // D s_13_11: create-bits s_13_9 s_13_10
        let s_13_11: Bits = Bits::new(s_13_9, s_13_10);
        // D s_13_12: cast reint s_13_11 -> u8
        let s_13_12: u8 = (s_13_11.value() as u8);
        // D s_13_13: cast zx s_13_12 -> bv
        let s_13_13: Bits = Bits::new(s_13_12 as u128, 5u16);
        // D s_13_14: cast zx s_13_13 -> i
        let s_13_14: i128 = (s_13_13.value() as i128);
        // D s_13_15: cast reint s_13_14 -> i64
        let s_13_15: i64 = (s_13_14 as i64);
        // D s_13_16: write-var d <= s_13_15
        fn_state.d = s_13_15;
        // N s_13_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i64
        let s_14_0: i64 = fn_state.d;
        // D s_14_1: write-var dshadow#7857 <= s_14_0
        fn_state.dshadow_7857 = s_14_0;
        // D s_14_2: read-var Rn:u8
        let s_14_2: u8 = fn_state.Rn;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 4u16);
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (s_14_3.value() as i128);
        // D s_14_5: cast reint s_14_4 -> i64
        let s_14_5: i64 = (s_14_4 as i64);
        // D s_14_6: write-var n <= s_14_5
        fn_state.n = s_14_5;
        // C s_14_7: const #15s : i
        let s_14_7: i128 = 15;
        // D s_14_8: read-var n:i64
        let s_14_8: i64 = fn_state.n;
        // D s_14_9: cast zx s_14_8 -> i
        let s_14_9: i128 = (i128::try_from(s_14_8).unwrap());
        // D s_14_10: cmp-eq s_14_9 s_14_7
        let s_14_10: bool = ((s_14_9) == (s_14_7));
        // N s_14_11: branch s_14_10 b19 b15
        if s_14_10 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#322365 <= s_15_0
        fn_state.gs_322365 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#322365:u8
        let s_16_0: bool = fn_state.gs_322365;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var dshadow#7857:i64
        let s_17_0: i64 = fn_state.dshadow_7857;
        // D s_17_1: read-var add:u8
        let s_17_1: bool = fn_state.add;
        // D s_17_2: read-var esize:i64
        let s_17_2: i64 = fn_state.esize;
        // D s_17_3: read-var imm32:u32
        let s_17_3: u32 = fn_state.imm32;
        // D s_17_4: read-var n:i64
        let s_17_4: i64 = fn_state.n;
        // D s_17_5: call execute_aarch32_instrs_VSTR_Op_A_txt(s_17_1, s_17_0, s_17_2, s_17_3, s_17_4)
        let s_17_5: () = execute_aarch32_instrs_VSTR_Op_A_txt(
            state,
            tracer,
            s_17_1,
            s_17_0,
            s_17_2,
            s_17_3,
            s_17_4,
        );
        // N s_17_6: return
        return;
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
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call CurrentInstrSet(s_19_0)
        let s_19_1: u32 = CurrentInstrSet(state, tracer, s_19_0);
        // C s_19_2: const #1u : u32
        let s_19_2: u32 = 1;
        // S s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#322365 <= s_19_3
        fn_state.gs_322365 = s_19_3;
        // N s_19_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var size:u8
        let s_20_0: u8 = fn_state.size;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #2u : u8
        let s_20_2: u8 = 2;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 2u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var Vd:u8
        let s_21_0: u8 = fn_state.Vd;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 4u16);
        // D s_21_2: read-var D:u8
        let s_21_2: bool = fn_state.D;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cast reint s_21_1 -> u128
        let s_21_4: u128 = (s_21_1.value() as u128);
        // D s_21_5: size-of s_21_1
        let s_21_5: u16 = s_21_1.length();
        // D s_21_6: cast reint s_21_3 -> u128
        let s_21_6: u128 = (s_21_3.value() as u128);
        // D s_21_7: size-of s_21_3
        let s_21_7: u16 = s_21_3.length();
        // D s_21_8: lsl s_21_4 s_21_7
        let s_21_8: u128 = s_21_4 << s_21_7;
        // D s_21_9: or s_21_8 s_21_6
        let s_21_9: u128 = ((s_21_8) | (s_21_6));
        // D s_21_10: add s_21_5 s_21_7
        let s_21_10: u16 = (s_21_5 + s_21_7);
        // D s_21_11: create-bits s_21_9 s_21_10
        let s_21_11: Bits = Bits::new(s_21_9, s_21_10);
        // D s_21_12: cast reint s_21_11 -> u8
        let s_21_12: u8 = (s_21_11.value() as u8);
        // D s_21_13: cast zx s_21_12 -> bv
        let s_21_13: Bits = Bits::new(s_21_12 as u128, 5u16);
        // D s_21_14: cast zx s_21_13 -> i
        let s_21_14: i128 = (s_21_13.value() as i128);
        // D s_21_15: cast reint s_21_14 -> i64
        let s_21_15: i64 = (s_21_14 as i64);
        // D s_21_16: write-var d <= s_21_15
        fn_state.d = s_21_15;
        // N s_21_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var size:u8
        let s_22_0: u8 = fn_state.size;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #3u : u8
        let s_22_2: u8 = 3;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var D:u8
        let s_23_0: bool = fn_state.D;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // D s_23_2: read-var Vd:u8
        let s_23_2: u8 = fn_state.Vd;
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 4u16);
        // D s_23_4: cast reint s_23_1 -> u128
        let s_23_4: u128 = (s_23_1.value() as u128);
        // D s_23_5: size-of s_23_1
        let s_23_5: u16 = s_23_1.length();
        // D s_23_6: cast reint s_23_3 -> u128
        let s_23_6: u128 = (s_23_3.value() as u128);
        // D s_23_7: size-of s_23_3
        let s_23_7: u16 = s_23_3.length();
        // D s_23_8: lsl s_23_4 s_23_7
        let s_23_8: u128 = s_23_4 << s_23_7;
        // D s_23_9: or s_23_8 s_23_6
        let s_23_9: u128 = ((s_23_8) | (s_23_6));
        // D s_23_10: add s_23_5 s_23_7
        let s_23_10: u16 = (s_23_5 + s_23_7);
        // D s_23_11: create-bits s_23_9 s_23_10
        let s_23_11: Bits = Bits::new(s_23_9, s_23_10);
        // D s_23_12: cast reint s_23_11 -> u8
        let s_23_12: u8 = (s_23_11.value() as u8);
        // D s_23_13: cast zx s_23_12 -> bv
        let s_23_13: Bits = Bits::new(s_23_12 as u128, 5u16);
        // D s_23_14: cast zx s_23_13 -> i
        let s_23_14: i128 = (s_23_13.value() as i128);
        // D s_23_15: cast reint s_23_14 -> i64
        let s_23_15: i64 = (s_23_14 as i64);
        // D s_23_16: write-var d <= s_23_15
        fn_state.d = s_23_15;
        // N s_23_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #32s : i
        let s_25_0: i128 = 32;
        // C s_25_1: const #0s : i
        let s_25_1: i128 = 0;
        // C s_25_2: const #8s : i
        let s_25_2: i128 = 8;
        // C s_25_3: const #1s : i
        let s_25_3: i128 = 1;
        // D s_25_4: read-var imm8:u8
        let s_25_4: u8 = fn_state.imm8;
        // D s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 8u16);
        // D s_25_6: call place_slice(s_25_0, s_25_5, s_25_1, s_25_2, s_25_3)
        let s_25_6: Bits = place_slice(
            state,
            tracer,
            s_25_0,
            s_25_5,
            s_25_1,
            s_25_2,
            s_25_3,
        );
        // D s_25_7: cast reint s_25_6 -> u32
        let s_25_7: u32 = (s_25_6.value() as u32);
        // D s_25_8: write-var imm32 <= s_25_7
        fn_state.imm32 = s_25_7;
        // N s_25_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call InITBlock(s_27_0)
        let s_27_1: bool = InITBlock(state, tracer, s_27_0);
        // D s_27_2: write-var gs#322342 <= s_27_1
        fn_state.gs_322342 = s_27_1;
        // N s_27_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HaveFP16Ext(s_29_0)
        let s_29_1: bool = HaveFP16Ext(state, tracer, s_29_0);
        // S s_29_2: not s_29_1
        let s_29_2: bool = !s_29_1;
        // D s_29_3: write-var gs#322340 <= s_29_2
        fn_state.gs_322340 = s_29_2;
        // N s_29_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#322341 <= s_30_0
        fn_state.gs_322341 = s_30_0;
        // N s_30_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
