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
use execute_aarch32_instrs_MOV_rr_Op_A_txt::*;
use DecodeRegShift::*;
use ConditionPassed::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_MOV_rr_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op: u8,
    Rs: u8,
    Rdm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_298688: bool,
        gs_298687: bool,
        gs_298686: bool,
        op: u8,
        Rs: u8,
        Rdm: u8,
    }
    let fn_state = FunctionState {
        op,
        Rs,
        Rdm,
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
        let s_2_0: u8 = fn_state.op;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #2u : u8
        let s_2_2: u8 = 2;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b13 b3
        if s_2_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op:u8
        let s_3_0: u8 = fn_state.op;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #3u : u8
        let s_3_2: u8 = 3;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b12 b4
        if s_3_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var op:u8
        let s_4_0: u8 = fn_state.op;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_2: const #4u : u8
        let s_4_2: u8 = 4;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b11 b5
        if s_4_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var op:u8
        let s_5_0: u8 = fn_state.op;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #7u : u8
        let s_5_2: u8 = 7;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var gs#298686 <= s_5_4
        fn_state.gs_298686 = s_5_4;
        // N s_5_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#298686:u8
        let s_6_0: bool = fn_state.gs_298686;
        // D s_6_1: write-var gs#298687 <= s_6_0
        fn_state.gs_298687 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#298687:u8
        let s_7_0: bool = fn_state.gs_298687;
        // D s_7_1: write-var gs#298688 <= s_7_0
        fn_state.gs_298688 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#298688:u8
        let s_8_0: bool = fn_state.gs_298688;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var Rdm:u8
        let s_9_0: u8 = fn_state.Rdm;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (s_9_1.value() as i128);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var Rdm:u8
        let s_9_4: u8 = fn_state.Rdm;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 3u16);
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (s_9_5.value() as i128);
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // D s_9_8: read-var Rs:u8
        let s_9_8: u8 = fn_state.Rs;
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 3u16);
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (s_9_9.value() as i128);
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // C s_9_12: const #() : ()
        let s_9_12: () = ();
        // S s_9_13: call InITBlock(s_9_12)
        let s_9_13: bool = InITBlock(state, tracer, s_9_12);
        // S s_9_14: not s_9_13
        let s_9_14: bool = !s_9_13;
        // C s_9_15: const #2s : i
        let s_9_15: i128 = 2;
        // D s_9_16: read-var op:u8
        let s_9_16: u8 = fn_state.op;
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 4u16);
        // C s_9_18: const #1u : u64
        let s_9_18: u64 = 1;
        // D s_9_19: bit-extract s_9_17 s_9_15 s_9_18
        let s_9_19: Bits = (Bits::new(
            ((s_9_17) >> (s_9_15)).value(),
            u16::try_from(s_9_18).unwrap(),
        ));
        // D s_9_20: cast reint s_9_19 -> u8
        let s_9_20: bool = ((s_9_19.value()) != 0);
        // C s_9_21: const #0s : i
        let s_9_21: i128 = 0;
        // C s_9_22: const #0u : u64
        let s_9_22: u64 = 0;
        // D s_9_23: cast zx s_9_20 -> u64
        let s_9_23: u64 = (s_9_20 as u64);
        // C s_9_24: const #1u : u64
        let s_9_24: u64 = 1;
        // D s_9_25: and s_9_23 s_9_24
        let s_9_25: u64 = ((s_9_23) & (s_9_24));
        // D s_9_26: cmp-eq s_9_25 s_9_24
        let s_9_26: bool = ((s_9_25) == (s_9_24));
        // D s_9_27: lsl s_9_23 s_9_21
        let s_9_27: u64 = s_9_23 << s_9_21;
        // D s_9_28: or s_9_22 s_9_27
        let s_9_28: u64 = ((s_9_22) | (s_9_27));
        // D s_9_29: cmpl s_9_27
        let s_9_29: u64 = !s_9_27;
        // D s_9_30: and s_9_22 s_9_29
        let s_9_30: u64 = ((s_9_22) & (s_9_29));
        // D s_9_31: select s_9_26 s_9_28 s_9_30
        let s_9_31: u64 = if s_9_26 { s_9_28 } else { s_9_30 };
        // D s_9_32: cast trunc s_9_31 -> u8
        let s_9_32: bool = ((s_9_31) != 0);
        // C s_9_33: const #0s : i
        let s_9_33: i128 = 0;
        // D s_9_34: read-var op:u8
        let s_9_34: u8 = fn_state.op;
        // D s_9_35: cast zx s_9_34 -> bv
        let s_9_35: Bits = Bits::new(s_9_34 as u128, 4u16);
        // C s_9_36: const #1u : u64
        let s_9_36: u64 = 1;
        // D s_9_37: bit-extract s_9_35 s_9_33 s_9_36
        let s_9_37: Bits = (Bits::new(
            ((s_9_35) >> (s_9_33)).value(),
            u16::try_from(s_9_36).unwrap(),
        ));
        // D s_9_38: cast reint s_9_37 -> u8
        let s_9_38: bool = ((s_9_37.value()) != 0);
        // C s_9_39: const #0s : i
        let s_9_39: i128 = 0;
        // C s_9_40: const #0u : u64
        let s_9_40: u64 = 0;
        // D s_9_41: cast zx s_9_38 -> u64
        let s_9_41: u64 = (s_9_38 as u64);
        // C s_9_42: const #1u : u64
        let s_9_42: u64 = 1;
        // D s_9_43: and s_9_41 s_9_42
        let s_9_43: u64 = ((s_9_41) & (s_9_42));
        // D s_9_44: cmp-eq s_9_43 s_9_42
        let s_9_44: bool = ((s_9_43) == (s_9_42));
        // D s_9_45: lsl s_9_41 s_9_39
        let s_9_45: u64 = s_9_41 << s_9_39;
        // D s_9_46: or s_9_40 s_9_45
        let s_9_46: u64 = ((s_9_40) | (s_9_45));
        // D s_9_47: cmpl s_9_45
        let s_9_47: u64 = !s_9_45;
        // D s_9_48: and s_9_40 s_9_47
        let s_9_48: u64 = ((s_9_40) & (s_9_47));
        // D s_9_49: select s_9_44 s_9_46 s_9_48
        let s_9_49: u64 = if s_9_44 { s_9_46 } else { s_9_48 };
        // D s_9_50: cast trunc s_9_49 -> u8
        let s_9_50: bool = ((s_9_49) != 0);
        // D s_9_51: cast zx s_9_32 -> bv
        let s_9_51: Bits = Bits::new(s_9_32 as u128, 1u16);
        // D s_9_52: cast zx s_9_50 -> bv
        let s_9_52: Bits = Bits::new(s_9_50 as u128, 1u16);
        // D s_9_53: cast reint s_9_51 -> u128
        let s_9_53: u128 = (s_9_51.value() as u128);
        // D s_9_54: size-of s_9_51
        let s_9_54: u16 = s_9_51.length();
        // D s_9_55: cast reint s_9_52 -> u128
        let s_9_55: u128 = (s_9_52.value() as u128);
        // D s_9_56: size-of s_9_52
        let s_9_56: u16 = s_9_52.length();
        // D s_9_57: lsl s_9_53 s_9_56
        let s_9_57: u128 = s_9_53 << s_9_56;
        // D s_9_58: or s_9_57 s_9_55
        let s_9_58: u128 = ((s_9_57) | (s_9_55));
        // D s_9_59: add s_9_54 s_9_56
        let s_9_59: u16 = (s_9_54 + s_9_56);
        // D s_9_60: create-bits s_9_58 s_9_59
        let s_9_60: Bits = Bits::new(s_9_58, s_9_59);
        // D s_9_61: cast reint s_9_60 -> u8
        let s_9_61: u8 = (s_9_60.value() as u8);
        // D s_9_62: call DecodeRegShift(s_9_61)
        let s_9_62: u32 = DecodeRegShift(state, tracer, s_9_61);
        // D s_9_63: call execute_aarch32_instrs_MOV_rr_Op_A_txt(s_9_3, s_9_7, s_9_11, s_9_14, s_9_62)
        let s_9_63: () = execute_aarch32_instrs_MOV_rr_Op_A_txt(
            state,
            tracer,
            s_9_3,
            s_9_7,
            s_9_11,
            s_9_14,
            s_9_62,
        );
        // N s_9_64: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#298686 <= s_11_0
        fn_state.gs_298686 = s_11_0;
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#298687 <= s_12_0
        fn_state.gs_298687 = s_12_0;
        // N s_12_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#298688 <= s_13_0
        fn_state.gs_298688 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
