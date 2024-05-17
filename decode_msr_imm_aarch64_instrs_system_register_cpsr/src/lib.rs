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
use u_get_SCTLR_EL1_Type_UMA::*;
use IsInHost::*;
use HaveFeatEBEP::*;
use HaveDITExt::*;
use HaveFeatNMI::*;
use AArch64_SystemAccessTrap::*;
use CurrentSecurityState::*;
use HavePANExt::*;
use HaveSME::*;
use HaveUAOExt::*;
use execute_aarch64_instrs_system_register_cpsr::*;
use HaveMTEExt::*;
use HaveSSBSExt::*;
use AArch64_CheckSystemAccess::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveVirtHostExt::*;
use common::*;
pub fn decode_msr_imm_aarch64_instrs_system_register_cpsr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op2: u8,
    CRm: u8,
    op1: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_165373: bool,
        gs_165418: bool,
        gs_165354: bool,
        field: u32,
        gs_165355: bool,
        ga_263628: u8,
        gs_165420: bool,
        operand: u8,
        gs_165417: bool,
        gs_165374: bool,
        min_EL: u8,
        gs_165353: bool,
        need_secure: bool,
        gs_165422: bool,
        op2: u8,
        CRm: u8,
        op1: u8,
    }
    let fn_state = FunctionState {
        op2,
        CRm,
        op1,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var op1:u8
        let s_0_0: u8 = fn_state.op1;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 3u16);
        // C s_0_2: const #0u : u8
        let s_0_2: u8 = 0;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 3u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b112 b1
        if s_0_4 {
            return block_112(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#165353 <= s_1_0
        fn_state.gs_165353 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#165353:u8
        let s_2_0: bool = fn_state.gs_165353;
        // N s_2_1: branch s_2_0 b111 b3
        if s_2_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var op1:u8
        let s_3_0: u8 = fn_state.op1;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 3u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b110 b4
        if s_3_4 {
            return block_110(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#165354 <= s_4_0
        fn_state.gs_165354 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#165354:u8
        let s_5_0: bool = fn_state.gs_165354;
        // N s_5_1: branch s_5_0 b109 b6
        if s_5_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var op1:u8
        let s_6_0: u8 = fn_state.op1;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_2: const #0u : u8
        let s_6_2: u8 = 0;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 3u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b108 b7
        if s_6_4 {
            return block_108(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#165355 <= s_7_0
        fn_state.gs_165355 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#165355:u8
        let s_8_0: bool = fn_state.gs_165355;
        // N s_8_1: branch s_8_0 b107 b9
        if s_8_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: u8 = 0;
        // D s_9_1: read-var op1:u8
        let s_9_1: u8 = fn_state.op1;
        // C s_9_2: const #4u : u8
        let s_9_2: u8 = 4;
        // D s_9_3: read-var CRm:u8
        let s_9_3: u8 = fn_state.CRm;
        // D s_9_4: read-var op2:u8
        let s_9_4: u8 = fn_state.op2;
        // C s_9_5: const #31u : u8
        let s_9_5: u8 = 31;
        // C s_9_6: const #0u : u8
        let s_9_6: bool = false;
        // D s_9_7: call AArch64_CheckSystemAccess(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5, s_9_6)
        let s_9_7: () = AArch64_CheckSystemAccess(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
            s_9_6,
        );
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var need_secure <= s_10_0
        fn_state.need_secure = s_10_0;
        // D s_10_2: read-var op1:u8
        let s_10_2: u8 = fn_state.op1;
        // C s_10_3: const #1s : i
        let s_10_3: i128 = 1;
        // D s_10_4: cast zx s_10_2 -> bv
        let s_10_4: Bits = Bits::new(s_10_2 as u128, 3u16);
        // C s_10_5: const #1s : i64
        let s_10_5: i64 = 1;
        // C s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // C s_10_7: const #1s : i
        let s_10_7: i128 = 1;
        // C s_10_8: add s_10_7 s_10_6
        let s_10_8: i128 = (s_10_7 + s_10_6);
        // D s_10_9: bit-extract s_10_4 s_10_3 s_10_8
        let s_10_9: Bits = (Bits::new(
            ((s_10_4) >> (s_10_3)).value(),
            u16::try_from(s_10_8).unwrap(),
        ));
        // D s_10_10: cast reint s_10_9 -> u8
        let s_10_10: u8 = (s_10_9.value() as u8);
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 2u16);
        // C s_10_12: const #0u : u8
        let s_10_12: u8 = 0;
        // C s_10_13: cast zx s_10_12 -> bv
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 2u16);
        // D s_10_14: cmp-eq s_10_11 s_10_13
        let s_10_14: bool = ((s_10_11) == (s_10_13));
        // D s_10_15: not s_10_14
        let s_10_15: bool = !s_10_14;
        // N s_10_16: branch s_10_15 b94 b11
        if s_10_15 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #440u : u32
        let s_11_0: u32 = 440;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: write-var min_EL <= s_11_1
        fn_state.min_EL = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var min_EL:u8
        let s_12_0: u8 = fn_state.min_EL;
        // C s_12_1: const #16975u : u32
        let s_12_1: u32 = 16975;
        // D s_12_2: read-reg s_12_1:u8
        let s_12_2: u8 = {
            let value = state.read_register::<u8>(s_12_1 as isize);
            tracer.read_register(s_12_1 as isize, value);
            value
        };
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 2u16);
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (s_12_3.value() as i128);
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: cast zx s_12_0 -> bv
        let s_12_6: Bits = Bits::new(s_12_0 as u128, 2u16);
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (s_12_6.value() as i128);
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: cast zx s_12_5 -> i
        let s_12_9: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_10: cast zx s_12_8 -> i
        let s_12_10: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_11: cmp-lt s_12_9 s_12_10
        let s_12_11: bool = ((s_12_9) < (s_12_10));
        // N s_12_12: branch s_12_11 b93 b13
        if s_12_11 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var need_secure:u8
        let s_13_0: bool = fn_state.need_secure;
        // N s_13_1: branch s_13_0 b92 b14
        if s_13_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#165373 <= s_14_0
        fn_state.gs_165373 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#165373:u8
        let s_15_0: bool = fn_state.gs_165373;
        // D s_15_1: write-var gs#165374 <= s_15_0
        fn_state.gs_165374 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#165374:u8
        let s_16_0: bool = fn_state.gs_165374;
        // N s_16_1: branch s_16_0 b91 b17
        if s_16_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var CRm:u8
        let s_17_0: u8 = fn_state.CRm;
        // D s_17_1: write-var operand <= s_17_0
        fn_state.operand = s_17_0;
        // D s_17_2: read-var op1:u8
        let s_17_2: u8 = fn_state.op1;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 3u16);
        // D s_17_4: read-var op2:u8
        let s_17_4: u8 = fn_state.op2;
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 3u16);
        // D s_17_6: cast reint s_17_3 -> u128
        let s_17_6: u128 = (s_17_3.value() as u128);
        // D s_17_7: size-of s_17_3
        let s_17_7: u16 = s_17_3.length();
        // D s_17_8: cast reint s_17_5 -> u128
        let s_17_8: u128 = (s_17_5.value() as u128);
        // D s_17_9: size-of s_17_5
        let s_17_9: u16 = s_17_5.length();
        // D s_17_10: lsl s_17_6 s_17_9
        let s_17_10: u128 = s_17_6 << s_17_9;
        // D s_17_11: or s_17_10 s_17_8
        let s_17_11: u128 = ((s_17_10) | (s_17_8));
        // D s_17_12: add s_17_7 s_17_9
        let s_17_12: u16 = (s_17_7 + s_17_9);
        // D s_17_13: create-bits s_17_11 s_17_12
        let s_17_13: Bits = Bits::new(s_17_11, s_17_12);
        // D s_17_14: cast reint s_17_13 -> u8
        let s_17_14: u8 = (s_17_13.value() as u8);
        // D s_17_15: write-var ga#263628 <= s_17_14
        fn_state.ga_263628 = s_17_14;
        // D s_17_16: read-var ga#263628:u8
        let s_17_16: u8 = fn_state.ga_263628;
        // D s_17_17: cast zx s_17_16 -> bv
        let s_17_17: Bits = Bits::new(s_17_16 as u128, 6u16);
        // C s_17_18: const #3u : u8
        let s_17_18: u8 = 3;
        // C s_17_19: cast zx s_17_18 -> bv
        let s_17_19: Bits = Bits::new(s_17_18 as u128, 6u16);
        // D s_17_20: cmp-eq s_17_17 s_17_19
        let s_17_20: bool = ((s_17_17) == (s_17_19));
        // D s_17_21: not s_17_20
        let s_17_21: bool = !s_17_20;
        // N s_17_22: branch s_17_21 b42 b18
        if s_17_21 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HaveUAOExt(s_18_0)
        let s_18_1: bool = HaveUAOExt(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // N s_18_3: branch s_18_2 b41 b19
        if s_18_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #3u : u32
        let s_19_0: u32 = 3;
        // D s_19_1: write-var field <= s_19_0
        fn_state.field = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #16975u : u32
        let s_20_0: u32 = 16975;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 2u16);
        // C s_20_3: const #448u : u32
        let s_20_3: u32 = 448;
        // D s_20_4: read-reg s_20_3:u8
        let s_20_4: u8 = {
            let value = state.read_register::<u8>(s_20_3 as isize);
            tracer.read_register(s_20_3 as isize, value);
            value
        };
        // D s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 2u16);
        // D s_20_6: cmp-eq s_20_2 s_20_5
        let s_20_6: bool = ((s_20_2) == (s_20_5));
        // N s_20_7: branch s_20_6 b37 b21
        if s_20_6 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#165418 <= s_21_0
        fn_state.gs_165418 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#165418:u8
        let s_22_0: bool = fn_state.gs_165418;
        // N s_22_1: branch s_22_0 b25 b23
        if s_22_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var field:u32
        let s_24_0: u32 = fn_state.field;
        // D s_24_1: read-var operand:u8
        let s_24_1: u8 = fn_state.operand;
        // D s_24_2: call execute_aarch64_instrs_system_register_cpsr(s_24_0, s_24_1)
        let s_24_2: () = execute_aarch64_instrs_system_register_cpsr(
            state,
            tracer,
            s_24_0,
            s_24_1,
        );
        // N s_24_3: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call IsInHost(s_25_0)
        let s_25_1: bool = IsInHost(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b36 b26
        if s_25_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #90272u : u32
        let s_26_0: u32 = 90272;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_SCTLR_EL1_Type_UMA(s_26_1)
        let s_26_2: bool = u_get_SCTLR_EL1_Type_UMA(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #0u : u8
        let s_26_4: bool = false;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // D s_26_7: write-var gs#165420 <= s_26_6
        fn_state.gs_165420 = s_26_6;
        // N s_26_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#165420:u8
        let s_27_0: bool = fn_state.gs_165420;
        // N s_27_1: branch s_27_0 b30 b28
        if s_27_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b35 b31
        if s_30_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#165422 <= s_31_0
        fn_state.gs_165422 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#165422:u8
        let s_32_0: bool = fn_state.gs_165422;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #24u : u8
        let s_33_0: u8 = 24;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #440u : u32
        let s_33_5: u32 = 440;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24u : u8
        let s_34_0: u8 = 24;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #432u : u32
        let s_34_5: u32 = 432;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_TGE(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_35_1);
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // D s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#165422 <= s_35_6
        fn_state.gs_165422 = s_35_6;
        // N s_35_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#165420 <= s_36_0
        fn_state.gs_165420 = s_36_0;
        // N s_36_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var field:u32
        let s_37_0: u32 = fn_state.field;
        // C s_37_1: const #0u : u32
        let s_37_1: u32 = 0;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // N s_37_3: branch s_37_2 b40 b38
        if s_37_2 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var field:u32
        let s_38_0: u32 = fn_state.field;
        // C s_38_1: const #1u : u32
        let s_38_1: u32 = 1;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: write-var gs#165417 <= s_38_2
        fn_state.gs_165417 = s_38_2;
        // N s_38_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#165417:u8
        let s_39_0: bool = fn_state.gs_165417;
        // D s_39_1: write-var gs#165418 <= s_39_0
        fn_state.gs_165418 = s_39_0;
        // N s_39_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#165417 <= s_40_0
        fn_state.gs_165417 = s_40_0;
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: panic
        panic!("{:?}", ());
        // N s_41_1: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var ga#263628:u8
        let s_42_0: u8 = fn_state.ga_263628;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 6u16);
        // C s_42_2: const #4u : u8
        let s_42_2: u8 = 4;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 6u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: not s_42_4
        let s_42_5: bool = !s_42_4;
        // N s_42_6: branch s_42_5 b46 b43
        if s_42_5 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call HavePANExt(s_43_0)
        let s_43_1: bool = HavePANExt(state, tracer, s_43_0);
        // S s_43_2: not s_43_1
        let s_43_2: bool = !s_43_1;
        // N s_43_3: branch s_43_2 b45 b44
        if s_43_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #2u : u32
        let s_44_0: u32 = 2;
        // D s_44_1: write-var field <= s_44_0
        fn_state.field = s_44_0;
        // N s_44_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var ga#263628:u8
        let s_46_0: u8 = fn_state.ga_263628;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 6u16);
        // C s_46_2: const #5u : u8
        let s_46_2: u8 = 5;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 6u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: not s_46_4
        let s_46_5: bool = !s_46_4;
        // N s_46_6: branch s_46_5 b48 b47
        if s_46_5 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #12u : u32
        let s_47_0: u32 = 12;
        // D s_47_1: write-var field <= s_47_0
        fn_state.field = s_47_0;
        // N s_47_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var ga#263628:u8
        let s_48_0: u8 = fn_state.ga_263628;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 6u16);
        // C s_48_2: const #8u : u8
        let s_48_2: u8 = 8;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 6u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: not s_48_4
        let s_48_5: bool = !s_48_4;
        // N s_48_6: branch s_48_5 b59 b49
        if s_48_5 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var CRm:u8
        let s_49_0: u8 = fn_state.CRm;
        // C s_49_1: const #1s : i
        let s_49_1: i128 = 1;
        // D s_49_2: cast zx s_49_0 -> bv
        let s_49_2: Bits = Bits::new(s_49_0 as u128, 4u16);
        // C s_49_3: const #1s : i64
        let s_49_3: i64 = 1;
        // C s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // C s_49_5: const #2s : i
        let s_49_5: i128 = 2;
        // C s_49_6: add s_49_5 s_49_4
        let s_49_6: i128 = (s_49_5 + s_49_4);
        // D s_49_7: bit-extract s_49_2 s_49_1 s_49_6
        let s_49_7: Bits = (Bits::new(
            ((s_49_2) >> (s_49_1)).value(),
            u16::try_from(s_49_6).unwrap(),
        ));
        // D s_49_8: cast reint s_49_7 -> u8
        let s_49_8: u8 = (s_49_7.value() as u8);
        // D s_49_9: cast zx s_49_8 -> bv
        let s_49_9: Bits = Bits::new(s_49_8 as u128, 3u16);
        // C s_49_10: const #0u : u8
        let s_49_10: u8 = 0;
        // C s_49_11: cast zx s_49_10 -> bv
        let s_49_11: Bits = Bits::new(s_49_10 as u128, 3u16);
        // D s_49_12: cmp-eq s_49_9 s_49_11
        let s_49_12: bool = ((s_49_9) == (s_49_11));
        // D s_49_13: not s_49_12
        let s_49_13: bool = !s_49_12;
        // N s_49_14: branch s_49_13 b54 b50
        if s_49_13 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call HaveFeatNMI(s_50_0)
        let s_50_1: bool = HaveFeatNMI(state, tracer, s_50_0);
        // S s_50_2: not s_50_1
        let s_50_2: bool = !s_50_1;
        // N s_50_3: branch s_50_2 b53 b51
        if s_50_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #10u : u32
        let s_51_0: u32 = 10;
        // D s_51_1: write-var field <= s_51_0
        fn_state.field = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var CRm:u8
        let s_54_0: u8 = fn_state.CRm;
        // C s_54_1: const #1s : i
        let s_54_1: i128 = 1;
        // D s_54_2: cast zx s_54_0 -> bv
        let s_54_2: Bits = Bits::new(s_54_0 as u128, 4u16);
        // C s_54_3: const #1s : i64
        let s_54_3: i64 = 1;
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #2s : i
        let s_54_5: i128 = 2;
        // C s_54_6: add s_54_5 s_54_4
        let s_54_6: i128 = (s_54_5 + s_54_4);
        // D s_54_7: bit-extract s_54_2 s_54_1 s_54_6
        let s_54_7: Bits = (Bits::new(
            ((s_54_2) >> (s_54_1)).value(),
            u16::try_from(s_54_6).unwrap(),
        ));
        // D s_54_8: cast reint s_54_7 -> u8
        let s_54_8: u8 = (s_54_7.value() as u8);
        // D s_54_9: cast zx s_54_8 -> bv
        let s_54_9: Bits = Bits::new(s_54_8 as u128, 3u16);
        // C s_54_10: const #1u : u8
        let s_54_10: u8 = 1;
        // C s_54_11: cast zx s_54_10 -> bv
        let s_54_11: Bits = Bits::new(s_54_10 as u128, 3u16);
        // D s_54_12: cmp-eq s_54_9 s_54_11
        let s_54_12: bool = ((s_54_9) == (s_54_11));
        // D s_54_13: not s_54_12
        let s_54_13: bool = !s_54_12;
        // N s_54_14: branch s_54_13 b58 b55
        if s_54_13 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call HaveFeatEBEP(s_55_0)
        let s_55_1: bool = HaveFeatEBEP(state, tracer, s_55_0);
        // S s_55_2: not s_55_1
        let s_55_2: bool = !s_55_1;
        // N s_55_3: branch s_55_2 b57 b56
        if s_55_2 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #11u : u32
        let s_56_0: u32 = 11;
        // D s_56_1: write-var field <= s_56_0
        fn_state.field = s_56_0;
        // N s_56_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: panic
        panic!("{:?}", ());
        // N s_57_1: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var ga#263628:u8
        let s_59_0: u8 = fn_state.ga_263628;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 6u16);
        // C s_59_2: const #26u : u8
        let s_59_2: u8 = 26;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 6u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: not s_59_4
        let s_59_5: bool = !s_59_4;
        // N s_59_6: branch s_59_5 b63 b60
        if s_59_5 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call HaveDITExt(s_60_0)
        let s_60_1: bool = HaveDITExt(state, tracer, s_60_0);
        // S s_60_2: not s_60_1
        let s_60_2: bool = !s_60_1;
        // N s_60_3: branch s_60_2 b62 b61
        if s_60_2 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #4u : u32
        let s_61_0: u32 = 4;
        // D s_61_1: write-var field <= s_61_0
        fn_state.field = s_61_0;
        // N s_61_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var ga#263628:u8
        let s_63_0: u8 = fn_state.ga_263628;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 6u16);
        // C s_63_2: const #27u : u8
        let s_63_2: u8 = 27;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 6u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: not s_63_4
        let s_63_5: bool = !s_63_4;
        // N s_63_6: branch s_63_5 b78 b64
        if s_63_5 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var CRm:u8
        let s_64_0: u8 = fn_state.CRm;
        // C s_64_1: const #1s : i
        let s_64_1: i128 = 1;
        // D s_64_2: cast zx s_64_0 -> bv
        let s_64_2: Bits = Bits::new(s_64_0 as u128, 4u16);
        // C s_64_3: const #1s : i64
        let s_64_3: i64 = 1;
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #2s : i
        let s_64_5: i128 = 2;
        // C s_64_6: add s_64_5 s_64_4
        let s_64_6: i128 = (s_64_5 + s_64_4);
        // D s_64_7: bit-extract s_64_2 s_64_1 s_64_6
        let s_64_7: Bits = (Bits::new(
            ((s_64_2) >> (s_64_1)).value(),
            u16::try_from(s_64_6).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: u8 = (s_64_7.value() as u8);
        // D s_64_9: cast zx s_64_8 -> bv
        let s_64_9: Bits = Bits::new(s_64_8 as u128, 3u16);
        // C s_64_10: const #1u : u8
        let s_64_10: u8 = 1;
        // C s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 3u16);
        // D s_64_12: cmp-eq s_64_9 s_64_11
        let s_64_12: bool = ((s_64_9) == (s_64_11));
        // D s_64_13: not s_64_12
        let s_64_13: bool = !s_64_12;
        // N s_64_14: branch s_64_13 b69 b65
        if s_64_13 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call HaveSME(s_65_0)
        let s_65_1: bool = HaveSME(state, tracer, s_65_0);
        // S s_65_2: not s_65_1
        let s_65_2: bool = !s_65_1;
        // N s_65_3: branch s_65_2 b68 b66
        if s_65_2 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #7u : u32
        let s_66_0: u32 = 7;
        // D s_66_1: write-var field <= s_66_0
        fn_state.field = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var CRm:u8
        let s_69_0: u8 = fn_state.CRm;
        // C s_69_1: const #1s : i
        let s_69_1: i128 = 1;
        // D s_69_2: cast zx s_69_0 -> bv
        let s_69_2: Bits = Bits::new(s_69_0 as u128, 4u16);
        // C s_69_3: const #1s : i64
        let s_69_3: i64 = 1;
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #2s : i
        let s_69_5: i128 = 2;
        // C s_69_6: add s_69_5 s_69_4
        let s_69_6: i128 = (s_69_5 + s_69_4);
        // D s_69_7: bit-extract s_69_2 s_69_1 s_69_6
        let s_69_7: Bits = (Bits::new(
            ((s_69_2) >> (s_69_1)).value(),
            u16::try_from(s_69_6).unwrap(),
        ));
        // D s_69_8: cast reint s_69_7 -> u8
        let s_69_8: u8 = (s_69_7.value() as u8);
        // D s_69_9: cast zx s_69_8 -> bv
        let s_69_9: Bits = Bits::new(s_69_8 as u128, 3u16);
        // C s_69_10: const #2u : u8
        let s_69_10: u8 = 2;
        // C s_69_11: cast zx s_69_10 -> bv
        let s_69_11: Bits = Bits::new(s_69_10 as u128, 3u16);
        // D s_69_12: cmp-eq s_69_9 s_69_11
        let s_69_12: bool = ((s_69_9) == (s_69_11));
        // D s_69_13: not s_69_12
        let s_69_13: bool = !s_69_12;
        // N s_69_14: branch s_69_13 b73 b70
        if s_69_13 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call HaveSME(s_70_0)
        let s_70_1: bool = HaveSME(state, tracer, s_70_0);
        // S s_70_2: not s_70_1
        let s_70_2: bool = !s_70_1;
        // N s_70_3: branch s_70_2 b72 b71
        if s_70_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #8u : u32
        let s_71_0: u32 = 8;
        // D s_71_1: write-var field <= s_71_0
        fn_state.field = s_71_0;
        // N s_71_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: panic
        panic!("{:?}", ());
        // N s_72_1: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var CRm:u8
        let s_73_0: u8 = fn_state.CRm;
        // C s_73_1: const #1s : i
        let s_73_1: i128 = 1;
        // D s_73_2: cast zx s_73_0 -> bv
        let s_73_2: Bits = Bits::new(s_73_0 as u128, 4u16);
        // C s_73_3: const #1s : i64
        let s_73_3: i64 = 1;
        // C s_73_4: cast zx s_73_3 -> i
        let s_73_4: i128 = (i128::try_from(s_73_3).unwrap());
        // C s_73_5: const #2s : i
        let s_73_5: i128 = 2;
        // C s_73_6: add s_73_5 s_73_4
        let s_73_6: i128 = (s_73_5 + s_73_4);
        // D s_73_7: bit-extract s_73_2 s_73_1 s_73_6
        let s_73_7: Bits = (Bits::new(
            ((s_73_2) >> (s_73_1)).value(),
            u16::try_from(s_73_6).unwrap(),
        ));
        // D s_73_8: cast reint s_73_7 -> u8
        let s_73_8: u8 = (s_73_7.value() as u8);
        // D s_73_9: cast zx s_73_8 -> bv
        let s_73_9: Bits = Bits::new(s_73_8 as u128, 3u16);
        // C s_73_10: const #3u : u8
        let s_73_10: u8 = 3;
        // C s_73_11: cast zx s_73_10 -> bv
        let s_73_11: Bits = Bits::new(s_73_10 as u128, 3u16);
        // D s_73_12: cmp-eq s_73_9 s_73_11
        let s_73_12: bool = ((s_73_9) == (s_73_11));
        // D s_73_13: not s_73_12
        let s_73_13: bool = !s_73_12;
        // N s_73_14: branch s_73_13 b77 b74
        if s_73_13 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call HaveSME(s_74_0)
        let s_74_1: bool = HaveSME(state, tracer, s_74_0);
        // S s_74_2: not s_74_1
        let s_74_2: bool = !s_74_1;
        // N s_74_3: branch s_74_2 b76 b75
        if s_74_2 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #9u : u32
        let s_75_0: u32 = 9;
        // D s_75_1: write-var field <= s_75_0
        fn_state.field = s_75_0;
        // N s_75_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: panic
        panic!("{:?}", ());
        // N s_77_1: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var ga#263628:u8
        let s_78_0: u8 = fn_state.ga_263628;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 6u16);
        // C s_78_2: const #28u : u8
        let s_78_2: u8 = 28;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 6u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: not s_78_4
        let s_78_5: bool = !s_78_4;
        // N s_78_6: branch s_78_5 b82 b79
        if s_78_5 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call HaveMTEExt(s_79_0)
        let s_79_1: bool = HaveMTEExt(state, tracer, s_79_0);
        // S s_79_2: not s_79_1
        let s_79_2: bool = !s_79_1;
        // N s_79_3: branch s_79_2 b81 b80
        if s_79_2 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #6u : u32
        let s_80_0: u32 = 6;
        // D s_80_1: write-var field <= s_80_0
        fn_state.field = s_80_0;
        // N s_80_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: panic
        panic!("{:?}", ());
        // N s_81_1: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var ga#263628:u8
        let s_82_0: u8 = fn_state.ga_263628;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 6u16);
        // C s_82_2: const #30u : u8
        let s_82_2: u8 = 30;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 6u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: not s_82_4
        let s_82_5: bool = !s_82_4;
        // N s_82_6: branch s_82_5 b84 b83
        if s_82_5 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u32
        let s_83_0: u32 = 0;
        // D s_83_1: write-var field <= s_83_0
        fn_state.field = s_83_0;
        // N s_83_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var ga#263628:u8
        let s_84_0: u8 = fn_state.ga_263628;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 6u16);
        // C s_84_2: const #31u : u8
        let s_84_2: u8 = 31;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 6u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: not s_84_4
        let s_84_5: bool = !s_84_4;
        // N s_84_6: branch s_84_5 b86 b85
        if s_84_5 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #1u : u32
        let s_85_0: u32 = 1;
        // D s_85_1: write-var field <= s_85_0
        fn_state.field = s_85_0;
        // N s_85_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var ga#263628:u8
        let s_86_0: u8 = fn_state.ga_263628;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 6u16);
        // C s_86_2: const #25u : u8
        let s_86_2: u8 = 25;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 6u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: not s_86_4
        let s_86_5: bool = !s_86_4;
        // N s_86_6: branch s_86_5 b90 b87
        if s_86_5 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call HaveSSBSExt(s_87_0)
        let s_87_1: bool = HaveSSBSExt(state, tracer, s_87_0);
        // S s_87_2: not s_87_1
        let s_87_2: bool = !s_87_1;
        // N s_87_3: branch s_87_2 b89 b88
        if s_87_2 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #5u : u32
        let s_88_0: u32 = 5;
        // D s_88_1: write-var field <= s_88_0
        fn_state.field = s_88_0;
        // N s_88_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_89_0: panic
        panic!("{:?}", ());
        // N s_89_1: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_90_0: panic
        panic!("{:?}", ());
        // N s_90_1: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: panic
        panic!("{:?}", ());
        // N s_91_1: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #() : ()
        let s_92_0: () = ();
        // S s_92_1: call CurrentSecurityState(s_92_0)
        let s_92_1: u32 = CurrentSecurityState(state, tracer, s_92_0);
        // C s_92_2: const #3u : u32
        let s_92_2: u32 = 3;
        // S s_92_3: cmp-eq s_92_1 s_92_2
        let s_92_3: bool = ((s_92_1) == (s_92_2));
        // D s_92_4: write-var gs#165373 <= s_92_3
        fn_state.gs_165373 = s_92_3;
        // N s_92_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#165374 <= s_93_0
        fn_state.gs_165374 = s_93_0;
        // N s_93_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var op1:u8
        let s_94_0: u8 = fn_state.op1;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 3u16);
        // C s_94_2: const #2u : u8
        let s_94_2: u8 = 2;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 3u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: not s_94_4
        let s_94_5: bool = !s_94_4;
        // N s_94_6: branch s_94_5 b96 b95
        if s_94_5 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #440u : u32
        let s_95_0: u32 = 440;
        // D s_95_1: read-reg s_95_0:u8
        let s_95_1: u8 = {
            let value = state.read_register::<u8>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: write-var min_EL <= s_95_1
        fn_state.min_EL = s_95_1;
        // N s_95_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var op1:u8
        let s_96_0: u8 = fn_state.op1;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 3u16);
        // C s_96_2: const #3u : u8
        let s_96_2: u8 = 3;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 3u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: not s_96_4
        let s_96_5: bool = !s_96_4;
        // N s_96_6: branch s_96_5 b98 b97
        if s_96_5 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #448u : u32
        let s_97_0: u32 = 448;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: write-var min_EL <= s_97_1
        fn_state.min_EL = s_97_1;
        // N s_97_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var op1:u8
        let s_98_0: u8 = fn_state.op1;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 3u16);
        // C s_98_2: const #4u : u8
        let s_98_2: u8 = 4;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 3u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: not s_98_4
        let s_98_5: bool = !s_98_4;
        // N s_98_6: branch s_98_5 b100 b99
        if s_98_5 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #432u : u32
        let s_99_0: u32 = 432;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: write-var min_EL <= s_99_1
        fn_state.min_EL = s_99_1;
        // N s_99_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var op1:u8
        let s_100_0: u8 = fn_state.op1;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 3u16);
        // C s_100_2: const #5u : u8
        let s_100_2: u8 = 5;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 3u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: not s_100_4
        let s_100_5: bool = !s_100_4;
        // N s_100_6: branch s_100_5 b104 b101
        if s_100_5 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call HaveVirtHostExt(s_101_0)
        let s_101_1: bool = HaveVirtHostExt(state, tracer, s_101_0);
        // S s_101_2: not s_101_1
        let s_101_2: bool = !s_101_1;
        // N s_101_3: branch s_101_2 b103 b102
        if s_101_2 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #432u : u32
        let s_102_0: u32 = 432;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: write-var min_EL <= s_102_1
        fn_state.min_EL = s_102_1;
        // N s_102_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: panic
        panic!("{:?}", ());
        // N s_103_1: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var op1:u8
        let s_104_0: u8 = fn_state.op1;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 3u16);
        // C s_104_2: const #6u : u8
        let s_104_2: u8 = 6;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 3u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: not s_104_4
        let s_104_5: bool = !s_104_4;
        // N s_104_6: branch s_104_5 b106 b105
        if s_104_5 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #424u : u32
        let s_105_0: u32 = 424;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: write-var min_EL <= s_105_1
        fn_state.min_EL = s_105_1;
        // N s_105_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #440u : u32
        let s_106_0: u32 = 440;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: write-var min_EL <= s_106_1
        fn_state.min_EL = s_106_1;
        // C s_106_3: const #1u : u8
        let s_106_3: bool = true;
        // D s_106_4: write-var need_secure <= s_106_3
        fn_state.need_secure = s_106_3;
        // N s_106_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: panic
        panic!("{:?}", ());
        // N s_107_1: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var op2:u8
        let s_108_0: u8 = fn_state.op2;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 3u16);
        // C s_108_2: const #2u : u8
        let s_108_2: u8 = 2;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 3u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#165355 <= s_108_4
        fn_state.gs_165355 = s_108_4;
        // N s_108_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_109_0: panic
        panic!("{:?}", ());
        // N s_109_1: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var op2:u8
        let s_110_0: u8 = fn_state.op2;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 3u16);
        // C s_110_2: const #1u : u8
        let s_110_2: u8 = 1;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 3u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#165354 <= s_110_4
        fn_state.gs_165354 = s_110_4;
        // N s_110_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: panic
        panic!("{:?}", ());
        // N s_111_1: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var op2:u8
        let s_112_0: u8 = fn_state.op2;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 3u16);
        // C s_112_2: const #0u : u8
        let s_112_2: u8 = 0;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 3u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#165353 <= s_112_4
        fn_state.gs_165353 = s_112_4;
        // N s_112_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
