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
use VFPExpandImm::*;
use u_get_FPSCR_Type_length::*;
use u_get_FPSCR_Type_Stride::*;
use ConditionPassed::*;
use HaveFP16Ext::*;
use InITBlock::*;
use FPSCR_read__1::*;
use Zeros::*;
use execute_aarch32_instrs_VMOV_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VMOV_i_T2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    imm4H: u8,
    Vd: u8,
    size: u8,
    imm4L: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        imm64: u64,
        gs_312964: bool,
        gs_312967: bool,
        single_register: bool,
        regs: i64,
        d: i64,
        gs_312965: bool,
        gs_312966: bool,
        D: bool,
        imm4H: u8,
        Vd: u8,
        size: u8,
        imm4L: u8,
    }
    let fn_state = FunctionState {
        D,
        imm4H,
        Vd,
        size,
        imm4L,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call FPSCR_read__1(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_2_0);
        // S s_2_2: call _get_FPSCR_Type_length(s_2_1)
        let s_2_2: u8 = u_get_FPSCR_Type_length(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 3u16);
        // C s_2_4: const #0u : u8
        let s_2_4: u8 = 0;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 3u16);
        // S s_2_6: cmp-ne s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) != (s_2_5));
        // N s_2_7: branch s_2_6 b27 b3
        if s_2_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call FPSCR_read__1(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_3_0);
        // S s_3_2: call _get_FPSCR_Type_Stride(s_3_1)
        let s_3_2: u8 = u_get_FPSCR_Type_Stride(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // S s_3_6: cmp-ne s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) != (s_3_5));
        // D s_3_7: write-var gs#312964 <= s_3_6
        fn_state.gs_312964 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#312964:u8
        let s_4_0: bool = fn_state.gs_312964;
        // N s_4_1: branch s_4_0 b26 b5
        if s_4_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b25 b6
        if s_5_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var size:u8
        let s_6_0: u8 = fn_state.size;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #1u : u8
        let s_6_2: u8 = 1;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 2u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b24 b7
        if s_6_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#312965 <= s_7_0
        fn_state.gs_312965 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#312965:u8
        let s_8_0: bool = fn_state.gs_312965;
        // D s_8_1: write-var gs#312966 <= s_8_0
        fn_state.gs_312966 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#312966:u8
        let s_9_0: bool = fn_state.gs_312966;
        // N s_9_1: branch s_9_0 b23 b10
        if s_9_0 {
            return block_23(state, tracer, fn_state);
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
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b22 b11
        if s_10_4 {
            return block_22(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#312967 <= s_11_0
        fn_state.gs_312967 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#312967:u8
        let s_12_0: bool = fn_state.gs_312967;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var size:u8
        let s_13_0: u8 = fn_state.size;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #3u : u8
        let s_13_2: u8 = 3;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-ne s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) != (s_13_3));
        // D s_13_5: write-var single_register <= s_13_4
        fn_state.single_register = s_13_4;
        // C s_13_6: const #1s : i64
        let s_13_6: i64 = 1;
        // D s_13_7: write-var regs <= s_13_6
        fn_state.regs = s_13_6;
        // D s_13_8: read-var size:u8
        let s_13_8: u8 = fn_state.size;
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 2u16);
        // C s_13_10: const #1u : u8
        let s_13_10: u8 = 1;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 2u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: not s_13_12
        let s_13_13: bool = !s_13_12;
        // N s_13_14: branch s_13_13 b16 b14
        if s_13_13 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var Vd:u8
        let s_14_0: u8 = fn_state.Vd;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 4u16);
        // D s_14_2: read-var D:u8
        let s_14_2: bool = fn_state.D;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
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
        // D s_14_17: read-var imm4H:u8
        let s_14_17: u8 = fn_state.imm4H;
        // D s_14_18: cast zx s_14_17 -> bv
        let s_14_18: Bits = Bits::new(s_14_17 as u128, 4u16);
        // D s_14_19: read-var imm4L:u8
        let s_14_19: u8 = fn_state.imm4L;
        // D s_14_20: cast zx s_14_19 -> bv
        let s_14_20: Bits = Bits::new(s_14_19 as u128, 4u16);
        // D s_14_21: cast reint s_14_18 -> u128
        let s_14_21: u128 = (s_14_18.value() as u128);
        // D s_14_22: size-of s_14_18
        let s_14_22: u16 = s_14_18.length();
        // D s_14_23: cast reint s_14_20 -> u128
        let s_14_23: u128 = (s_14_20.value() as u128);
        // D s_14_24: size-of s_14_20
        let s_14_24: u16 = s_14_20.length();
        // D s_14_25: lsl s_14_21 s_14_24
        let s_14_25: u128 = s_14_21 << s_14_24;
        // D s_14_26: or s_14_25 s_14_23
        let s_14_26: u128 = ((s_14_25) | (s_14_23));
        // D s_14_27: add s_14_22 s_14_24
        let s_14_27: u16 = (s_14_22 + s_14_24);
        // D s_14_28: create-bits s_14_26 s_14_27
        let s_14_28: Bits = Bits::new(s_14_26, s_14_27);
        // D s_14_29: cast reint s_14_28 -> u8
        let s_14_29: u8 = (s_14_28.value() as u8);
        // C s_14_30: const #16s : i64
        let s_14_30: i64 = 16;
        // D s_14_31: call VFPExpandImm(s_14_29, s_14_30)
        let s_14_31: Bits = VFPExpandImm(state, tracer, s_14_29, s_14_30);
        // D s_14_32: cast reint s_14_31 -> u16
        let s_14_32: u16 = (s_14_31.value() as u16);
        // C s_14_33: const #16s : i
        let s_14_33: i128 = 16;
        // S s_14_34: call Zeros(s_14_33)
        let s_14_34: Bits = Zeros(state, tracer, s_14_33);
        // S s_14_35: cast reint s_14_34 -> u16
        let s_14_35: u16 = (s_14_34.value() as u16);
        // S s_14_36: cast zx s_14_35 -> bv
        let s_14_36: Bits = Bits::new(s_14_35 as u128, 16u16);
        // D s_14_37: cast zx s_14_32 -> bv
        let s_14_37: Bits = Bits::new(s_14_32 as u128, 16u16);
        // S s_14_38: cast reint s_14_36 -> u128
        let s_14_38: u128 = (s_14_36.value() as u128);
        // D s_14_39: size-of s_14_36
        let s_14_39: u16 = s_14_36.length();
        // D s_14_40: cast reint s_14_37 -> u128
        let s_14_40: u128 = (s_14_37.value() as u128);
        // D s_14_41: size-of s_14_37
        let s_14_41: u16 = s_14_37.length();
        // D s_14_42: lsl s_14_38 s_14_41
        let s_14_42: u128 = s_14_38 << s_14_41;
        // D s_14_43: or s_14_42 s_14_40
        let s_14_43: u128 = ((s_14_42) | (s_14_40));
        // D s_14_44: add s_14_39 s_14_41
        let s_14_44: u16 = (s_14_39 + s_14_41);
        // D s_14_45: create-bits s_14_43 s_14_44
        let s_14_45: Bits = Bits::new(s_14_43, s_14_44);
        // D s_14_46: cast reint s_14_45 -> u32
        let s_14_46: u32 = (s_14_45.value() as u32);
        // D s_14_47: write-var imm32 <= s_14_46
        fn_state.imm32 = s_14_46;
        // N s_14_48: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var regs:i64
        let s_15_0: i64 = fn_state.regs;
        // D s_15_1: read-var d:i64
        let s_15_1: i64 = fn_state.d;
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // D s_15_3: read-var imm32:u32
        let s_15_3: u32 = fn_state.imm32;
        // D s_15_4: read-var imm64:u64
        let s_15_4: u64 = fn_state.imm64;
        // D s_15_5: read-var single_register:u8
        let s_15_5: bool = fn_state.single_register;
        // D s_15_6: call execute_aarch32_instrs_VMOV_i_Op_A_txt(s_15_2, s_15_1, s_15_3, s_15_4, s_15_0, s_15_5)
        let s_15_6: () = execute_aarch32_instrs_VMOV_i_Op_A_txt(
            state,
            tracer,
            s_15_2,
            s_15_1,
            s_15_3,
            s_15_4,
            s_15_0,
            s_15_5,
        );
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var size:u8
        let s_16_0: u8 = fn_state.size;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 2u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
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
        // D s_17_0: read-var Vd:u8
        let s_17_0: u8 = fn_state.Vd;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 4u16);
        // D s_17_2: read-var D:u8
        let s_17_2: bool = fn_state.D;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cast reint s_17_1 -> u128
        let s_17_4: u128 = (s_17_1.value() as u128);
        // D s_17_5: size-of s_17_1
        let s_17_5: u16 = s_17_1.length();
        // D s_17_6: cast reint s_17_3 -> u128
        let s_17_6: u128 = (s_17_3.value() as u128);
        // D s_17_7: size-of s_17_3
        let s_17_7: u16 = s_17_3.length();
        // D s_17_8: lsl s_17_4 s_17_7
        let s_17_8: u128 = s_17_4 << s_17_7;
        // D s_17_9: or s_17_8 s_17_6
        let s_17_9: u128 = ((s_17_8) | (s_17_6));
        // D s_17_10: add s_17_5 s_17_7
        let s_17_10: u16 = (s_17_5 + s_17_7);
        // D s_17_11: create-bits s_17_9 s_17_10
        let s_17_11: Bits = Bits::new(s_17_9, s_17_10);
        // D s_17_12: cast reint s_17_11 -> u8
        let s_17_12: u8 = (s_17_11.value() as u8);
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 5u16);
        // D s_17_14: cast zx s_17_13 -> i
        let s_17_14: i128 = (s_17_13.value() as i128);
        // D s_17_15: cast reint s_17_14 -> i64
        let s_17_15: i64 = (s_17_14 as i64);
        // D s_17_16: write-var d <= s_17_15
        fn_state.d = s_17_15;
        // D s_17_17: read-var imm4H:u8
        let s_17_17: u8 = fn_state.imm4H;
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 4u16);
        // D s_17_19: read-var imm4L:u8
        let s_17_19: u8 = fn_state.imm4L;
        // D s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 4u16);
        // D s_17_21: cast reint s_17_18 -> u128
        let s_17_21: u128 = (s_17_18.value() as u128);
        // D s_17_22: size-of s_17_18
        let s_17_22: u16 = s_17_18.length();
        // D s_17_23: cast reint s_17_20 -> u128
        let s_17_23: u128 = (s_17_20.value() as u128);
        // D s_17_24: size-of s_17_20
        let s_17_24: u16 = s_17_20.length();
        // D s_17_25: lsl s_17_21 s_17_24
        let s_17_25: u128 = s_17_21 << s_17_24;
        // D s_17_26: or s_17_25 s_17_23
        let s_17_26: u128 = ((s_17_25) | (s_17_23));
        // D s_17_27: add s_17_22 s_17_24
        let s_17_27: u16 = (s_17_22 + s_17_24);
        // D s_17_28: create-bits s_17_26 s_17_27
        let s_17_28: Bits = Bits::new(s_17_26, s_17_27);
        // D s_17_29: cast reint s_17_28 -> u8
        let s_17_29: u8 = (s_17_28.value() as u8);
        // C s_17_30: const #32s : i64
        let s_17_30: i64 = 32;
        // D s_17_31: call VFPExpandImm(s_17_29, s_17_30)
        let s_17_31: Bits = VFPExpandImm(state, tracer, s_17_29, s_17_30);
        // D s_17_32: cast reint s_17_31 -> u32
        let s_17_32: u32 = (s_17_31.value() as u32);
        // D s_17_33: write-var imm32 <= s_17_32
        fn_state.imm32 = s_17_32;
        // N s_17_34: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var size:u8
        let s_18_0: u8 = fn_state.size;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 2u16);
        // C s_18_2: const #3u : u8
        let s_18_2: u8 = 3;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var D:u8
        let s_19_0: bool = fn_state.D;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // D s_19_2: read-var Vd:u8
        let s_19_2: u8 = fn_state.Vd;
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 4u16);
        // D s_19_4: cast reint s_19_1 -> u128
        let s_19_4: u128 = (s_19_1.value() as u128);
        // D s_19_5: size-of s_19_1
        let s_19_5: u16 = s_19_1.length();
        // D s_19_6: cast reint s_19_3 -> u128
        let s_19_6: u128 = (s_19_3.value() as u128);
        // D s_19_7: size-of s_19_3
        let s_19_7: u16 = s_19_3.length();
        // D s_19_8: lsl s_19_4 s_19_7
        let s_19_8: u128 = s_19_4 << s_19_7;
        // D s_19_9: or s_19_8 s_19_6
        let s_19_9: u128 = ((s_19_8) | (s_19_6));
        // D s_19_10: add s_19_5 s_19_7
        let s_19_10: u16 = (s_19_5 + s_19_7);
        // D s_19_11: create-bits s_19_9 s_19_10
        let s_19_11: Bits = Bits::new(s_19_9, s_19_10);
        // D s_19_12: cast reint s_19_11 -> u8
        let s_19_12: u8 = (s_19_11.value() as u8);
        // D s_19_13: cast zx s_19_12 -> bv
        let s_19_13: Bits = Bits::new(s_19_12 as u128, 5u16);
        // D s_19_14: cast zx s_19_13 -> i
        let s_19_14: i128 = (s_19_13.value() as i128);
        // D s_19_15: cast reint s_19_14 -> i64
        let s_19_15: i64 = (s_19_14 as i64);
        // D s_19_16: write-var d <= s_19_15
        fn_state.d = s_19_15;
        // D s_19_17: read-var imm4H:u8
        let s_19_17: u8 = fn_state.imm4H;
        // D s_19_18: cast zx s_19_17 -> bv
        let s_19_18: Bits = Bits::new(s_19_17 as u128, 4u16);
        // D s_19_19: read-var imm4L:u8
        let s_19_19: u8 = fn_state.imm4L;
        // D s_19_20: cast zx s_19_19 -> bv
        let s_19_20: Bits = Bits::new(s_19_19 as u128, 4u16);
        // D s_19_21: cast reint s_19_18 -> u128
        let s_19_21: u128 = (s_19_18.value() as u128);
        // D s_19_22: size-of s_19_18
        let s_19_22: u16 = s_19_18.length();
        // D s_19_23: cast reint s_19_20 -> u128
        let s_19_23: u128 = (s_19_20.value() as u128);
        // D s_19_24: size-of s_19_20
        let s_19_24: u16 = s_19_20.length();
        // D s_19_25: lsl s_19_21 s_19_24
        let s_19_25: u128 = s_19_21 << s_19_24;
        // D s_19_26: or s_19_25 s_19_23
        let s_19_26: u128 = ((s_19_25) | (s_19_23));
        // D s_19_27: add s_19_22 s_19_24
        let s_19_27: u16 = (s_19_22 + s_19_24);
        // D s_19_28: create-bits s_19_26 s_19_27
        let s_19_28: Bits = Bits::new(s_19_26, s_19_27);
        // D s_19_29: cast reint s_19_28 -> u8
        let s_19_29: u8 = (s_19_28.value() as u8);
        // C s_19_30: const #64s : i64
        let s_19_30: i64 = 64;
        // D s_19_31: call VFPExpandImm(s_19_29, s_19_30)
        let s_19_31: Bits = VFPExpandImm(state, tracer, s_19_29, s_19_30);
        // D s_19_32: cast reint s_19_31 -> u64
        let s_19_32: u64 = (s_19_31.value() as u64);
        // D s_19_33: write-var imm64 <= s_19_32
        fn_state.imm64 = s_19_32;
        // C s_19_34: const #1s : i64
        let s_19_34: i64 = 1;
        // D s_19_35: write-var regs <= s_19_34
        fn_state.regs = s_19_34;
        // N s_19_36: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call InITBlock(s_22_0)
        let s_22_1: bool = InITBlock(state, tracer, s_22_0);
        // D s_22_2: write-var gs#312967 <= s_22_1
        fn_state.gs_312967 = s_22_1;
        // N s_22_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveFP16Ext(s_24_0)
        let s_24_1: bool = HaveFP16Ext(state, tracer, s_24_0);
        // S s_24_2: not s_24_1
        let s_24_2: bool = !s_24_1;
        // D s_24_3: write-var gs#312965 <= s_24_2
        fn_state.gs_312965 = s_24_2;
        // N s_24_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#312966 <= s_25_0
        fn_state.gs_312966 = s_25_0;
        // N s_25_2: jump b9
        return block_9(state, tracer, fn_state);
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
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#312964 <= s_27_0
        fn_state.gs_312964 = s_27_0;
        // N s_27_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
