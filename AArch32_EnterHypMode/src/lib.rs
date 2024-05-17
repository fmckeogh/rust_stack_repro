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
use AArch32_ReportHypEntry::*;
use u_get_HSCTLR_Type_EE::*;
use Halted::*;
use ELR_hyp_write::*;
use u_get_SCRType_IRQ::*;
use u_get_SCRType_EA::*;
use AArch32_EnterHypModeInDebugState::*;
use CurrentSecurityState::*;
use SCR_GEN_read::*;
use u_get_HSCTLR_Type_TE::*;
use u_get_SCRType_FIQ::*;
use GetPSRFromPSTATE::*;
use SPSR_set::*;
use HaveSSBSExt::*;
use CheckExceptionCatch::*;
use integer_subrange::*;
use AArch32_WriteMode::*;
use EndOfInstruction::*;
use HVBAR_read::*;
use SynchronizeContext::*;
use u_get_HSCTLR_Type_DSSBS::*;
use ELUsingAArch32::*;
use BranchTo::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_EnterHypMode<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
    preferred_exception_return: u32,
    vect_offset: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_8960: bool,
        gs_8968: bool,
        gs_8970: bool,
        spsr: u32,
        gs_8969: bool,
        gs_8959: bool,
        gs_8966: bool,
        except: ProductTypeb7f99f96751e17c4,
        preferred_exception_return: u32,
        vect_offset: i128,
    }
    let fn_state = FunctionState {
        except,
        preferred_exception_return,
        vect_offset,
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
        // S s_0_1: call SynchronizeContext(s_0_0)
        let s_0_1: () = SynchronizeContext(state, tracer, s_0_0);
        // C s_0_2: const #432u : u32
        let s_0_2: u32 = 432;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #2u : u8
        let s_0_4: u8 = 2;
        // D s_0_5: cmp-lt s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) < (s_0_4));
        // N s_0_6: branch s_0_5 b36 b1
        if s_0_5 {
            return block_36(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#8959 <= s_1_0
        fn_state.gs_8959 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#8959:u8
        let s_2_0: bool = fn_state.gs_8959;
        // N s_2_1: branch s_2_0 b35 b3
        if s_2_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#8960 <= s_3_0
        fn_state.gs_8960 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#8960:u8
        let s_4_0: bool = fn_state.gs_8960;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #() : ()
        let s_4_2: () = ();
        // S s_4_3: call Halted(s_4_2)
        let s_4_3: bool = Halted(state, tracer, s_4_2);
        // N s_4_4: branch s_4_3 b34 b5
        if s_4_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // C s_5_1: const #0u : u32
        let s_5_1: u32 = 0;
        // S s_5_2: call GetPSRFromPSTATE(s_5_1, s_5_0)
        let s_5_2: Bits = GetPSRFromPSTATE(state, tracer, s_5_1, s_5_0);
        // S s_5_3: cast reint s_5_2 -> u32
        let s_5_3: u32 = (s_5_2.value() as u32);
        // D s_5_4: write-var spsr <= s_5_3
        fn_state.spsr = s_5_3;
        // D s_5_5: read-var except.1:struct
        let s_5_5: u32 = fn_state.except._1;
        // C s_5_6: const #31u : u32
        let s_5_6: u32 = 31;
        // D s_5_7: cmp-eq s_5_5 s_5_6
        let s_5_7: bool = ((s_5_5) == (s_5_6));
        // N s_5_8: branch s_5_7 b33 b6
        if s_5_7 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var except.1:struct
        let s_6_0: u32 = fn_state.except._1;
        // C s_6_1: const #40u : u32
        let s_6_1: u32 = 40;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: write-var gs#8966 <= s_6_2
        fn_state.gs_8966 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#8966:u8
        let s_7_0: bool = fn_state.gs_8966;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b32 b8
        if s_7_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #400u : u32
        let s_9_0: u32 = 400;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call AArch32_WriteMode(s_9_1)
        let s_9_2: () = AArch32_WriteMode(state, tracer, s_9_1);
        // C s_9_3: const #32s : i
        let s_9_3: i128 = 32;
        // D s_9_4: read-var spsr:u32
        let s_9_4: u32 = fn_state.spsr;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 32u16);
        // D s_9_6: call SPSR_set(s_9_3, s_9_5)
        let s_9_6: () = SPSR_set(state, tracer, s_9_3, s_9_5);
        // D s_9_7: read-var preferred_exception_return:u32
        let s_9_7: u32 = fn_state.preferred_exception_return;
        // D s_9_8: call ELR_hyp_write(s_9_7)
        let s_9_8: () = ELR_hyp_write(state, tracer, s_9_7);
        // C s_9_9: const #() : ()
        let s_9_9: () = ();
        // S s_9_10: call HSCTLR_read(s_9_9)
        let s_9_10: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_9_9);
        // S s_9_11: call _get_HSCTLR_Type_TE(s_9_10)
        let s_9_11: bool = u_get_HSCTLR_Type_TE(state, tracer, s_9_10);
        // C s_9_12: const #16993u : u32
        let s_9_12: u32 = 16993;
        // N s_9_13: write-reg s_9_12 <= s_9_11
        let s_9_13: () = {
            state.write_register::<bool>(s_9_12 as isize, s_9_11);
            tracer.write_register(s_9_12 as isize, s_9_11);
        };
        // C s_9_14: const #0u : u8
        let s_9_14: bool = false;
        // C s_9_15: const #16991u : u32
        let s_9_15: u32 = 16991;
        // N s_9_16: write-reg s_9_15 <= s_9_14
        let s_9_16: () = {
            state.write_register::<bool>(s_9_15 as isize, s_9_14);
            tracer.write_register(s_9_15 as isize, s_9_14);
        };
        // C s_9_17: const #424u : u32
        let s_9_17: u32 = 424;
        // D s_9_18: read-reg s_9_17:u8
        let s_9_18: u8 = {
            let value = state.read_register::<u8>(s_9_17 as isize);
            tracer.read_register(s_9_17 as isize, value);
            value
        };
        // C s_9_19: const #2u : u8
        let s_9_19: u8 = 2;
        // D s_9_20: cmp-lt s_9_18 s_9_19
        let s_9_20: bool = ((s_9_18) < (s_9_19));
        // D s_9_21: not s_9_20
        let s_9_21: bool = !s_9_20;
        // N s_9_22: branch s_9_21 b31 b10
        if s_9_21 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call SCR_GEN_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_10_0);
        // S s_10_2: call _get_SCRType_EA(s_10_1)
        let s_10_2: bool = u_get_SCRType_EA(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #0u : u8
        let s_10_4: bool = false;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // S s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#8968 <= s_10_6
        fn_state.gs_8968 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#8968:u8
        let s_11_0: bool = fn_state.gs_8968;
        // N s_11_1: branch s_11_0 b30 b12
        if s_11_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b29 b14
        if s_13_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SCR_GEN_read(s_14_0)
        let s_14_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_14_0);
        // S s_14_2: call _get_SCRType_IRQ(s_14_1)
        let s_14_2: bool = u_get_SCRType_IRQ(state, tracer, s_14_1);
        // S s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // C s_14_4: const #0u : u8
        let s_14_4: bool = false;
        // C s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 1u16);
        // S s_14_6: cmp-eq s_14_3 s_14_5
        let s_14_6: bool = ((s_14_3) == (s_14_5));
        // D s_14_7: write-var gs#8969 <= s_14_6
        fn_state.gs_8969 = s_14_6;
        // N s_14_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#8969:u8
        let s_15_0: bool = fn_state.gs_8969;
        // N s_15_1: branch s_15_0 b28 b16
        if s_15_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b27 b18
        if s_17_4 {
            return block_27(state, tracer, fn_state);
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
        // S s_18_1: call SCR_GEN_read(s_18_0)
        let s_18_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_18_0);
        // S s_18_2: call _get_SCRType_FIQ(s_18_1)
        let s_18_2: bool = u_get_SCRType_FIQ(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #0u : u8
        let s_18_4: bool = false;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // S s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#8970 <= s_18_6
        fn_state.gs_8970 = s_18_6;
        // N s_18_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#8970:u8
        let s_19_0: bool = fn_state.gs_8970;
        // N s_19_1: branch s_19_0 b26 b20
        if s_19_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call HSCTLR_read(s_21_0)
        let s_21_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_21_0);
        // S s_21_2: call _get_HSCTLR_Type_EE(s_21_1)
        let s_21_2: bool = u_get_HSCTLR_Type_EE(state, tracer, s_21_1);
        // C s_21_3: const #16974u : u32
        let s_21_3: u32 = 16974;
        // N s_21_4: write-reg s_21_3 <= s_21_2
        let s_21_4: () = {
            state.write_register::<bool>(s_21_3 as isize, s_21_2);
            tracer.write_register(s_21_3 as isize, s_21_2);
        };
        // C s_21_5: const #0u : u8
        let s_21_5: bool = false;
        // C s_21_6: const #16980u : u32
        let s_21_6: u32 = 16980;
        // N s_21_7: write-reg s_21_6 <= s_21_5
        let s_21_7: () = {
            state.write_register::<bool>(s_21_6 as isize, s_21_5);
            tracer.write_register(s_21_6 as isize, s_21_5);
        };
        // C s_21_8: const #0u : u8
        let s_21_8: u8 = 0;
        // C s_21_9: const #16981u : u32
        let s_21_9: u32 = 16981;
        // N s_21_10: write-reg s_21_9 <= s_21_8
        let s_21_10: () = {
            state.write_register::<u8>(s_21_9 as isize, s_21_8);
            tracer.write_register(s_21_9 as isize, s_21_8);
        };
        // C s_21_11: const #() : ()
        let s_21_11: () = ();
        // S s_21_12: call HaveSSBSExt(s_21_11)
        let s_21_12: bool = HaveSSBSExt(state, tracer, s_21_11);
        // N s_21_13: branch s_21_12 b25 b22
        if s_21_12 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // C s_23_1: const #() : ()
        let s_23_1: () = ();
        // S s_23_2: call HVBAR_read(s_23_1)
        let s_23_2: u32 = HVBAR_read(state, tracer, s_23_1);
        // C s_23_3: const #5s : i
        let s_23_3: i128 = 5;
        // S s_23_4: cast zx s_23_2 -> bv
        let s_23_4: Bits = Bits::new(s_23_2 as u128, 32u16);
        // C s_23_5: const #1s : i64
        let s_23_5: i64 = 1;
        // C s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // C s_23_7: const #26s : i
        let s_23_7: i128 = 26;
        // C s_23_8: add s_23_7 s_23_6
        let s_23_8: i128 = (s_23_7 + s_23_6);
        // D s_23_9: bit-extract s_23_4 s_23_3 s_23_8
        let s_23_9: Bits = (Bits::new(
            ((s_23_4) >> (s_23_3)).value(),
            u16::try_from(s_23_8).unwrap(),
        ));
        // D s_23_10: cast reint s_23_9 -> u27
        let s_23_10: u32 = (s_23_9.value() as u32);
        // C s_23_11: const #4s : i
        let s_23_11: i128 = 4;
        // C s_23_12: const #0s : i
        let s_23_12: i128 = 0;
        // D s_23_13: read-var vect_offset:i
        let s_23_13: i128 = fn_state.vect_offset;
        // D s_23_14: call integer_subrange(s_23_13, s_23_11, s_23_12)
        let s_23_14: Bits = integer_subrange(state, tracer, s_23_13, s_23_11, s_23_12);
        // D s_23_15: cast reint s_23_14 -> u8
        let s_23_15: u8 = (s_23_14.value() as u8);
        // D s_23_16: cast zx s_23_10 -> bv
        let s_23_16: Bits = Bits::new(s_23_10 as u128, 27u16);
        // D s_23_17: cast zx s_23_15 -> bv
        let s_23_17: Bits = Bits::new(s_23_15 as u128, 5u16);
        // D s_23_18: cast reint s_23_16 -> u128
        let s_23_18: u128 = (s_23_16.value() as u128);
        // D s_23_19: size-of s_23_16
        let s_23_19: u16 = s_23_16.length();
        // D s_23_20: cast reint s_23_17 -> u128
        let s_23_20: u128 = (s_23_17.value() as u128);
        // D s_23_21: size-of s_23_17
        let s_23_21: u16 = s_23_17.length();
        // D s_23_22: lsl s_23_18 s_23_21
        let s_23_22: u128 = s_23_18 << s_23_21;
        // D s_23_23: or s_23_22 s_23_20
        let s_23_23: u128 = ((s_23_22) | (s_23_20));
        // D s_23_24: add s_23_19 s_23_21
        let s_23_24: u16 = (s_23_19 + s_23_21);
        // D s_23_25: create-bits s_23_23 s_23_24
        let s_23_25: Bits = Bits::new(s_23_23, s_23_24);
        // D s_23_26: cast reint s_23_25 -> u32
        let s_23_26: u32 = (s_23_25.value() as u32);
        // D s_23_27: cast zx s_23_26 -> bv
        let s_23_27: Bits = Bits::new(s_23_26 as u128, 32u16);
        // C s_23_28: const #7u : u32
        let s_23_28: u32 = 7;
        // D s_23_29: call BranchTo(s_23_27, s_23_28, s_23_0)
        let s_23_29: () = BranchTo(state, tracer, s_23_27, s_23_28, s_23_0);
        // C s_23_30: const #1u : u8
        let s_23_30: bool = true;
        // S s_23_31: call CheckExceptionCatch(s_23_30)
        let s_23_31: () = CheckExceptionCatch(state, tracer, s_23_30);
        // N s_23_32: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EndOfInstruction(s_24_0)
        let s_24_1: () = EndOfInstruction(state, tracer, s_24_0);
        // N s_24_2: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HSCTLR_read(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_25_0);
        // S s_25_2: call _get_HSCTLR_Type_DSSBS(s_25_1)
        let s_25_2: bool = u_get_HSCTLR_Type_DSSBS(state, tracer, s_25_1);
        // C s_25_3: const #16992u : u32
        let s_25_3: u32 = 16992;
        // N s_25_4: write-reg s_25_3 <= s_25_2
        let s_25_4: () = {
            state.write_register::<bool>(s_25_3 as isize, s_25_2);
            tracer.write_register(s_25_3 as isize, s_25_2);
        };
        // N s_25_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // C s_26_1: const #16977u : u32
        let s_26_1: u32 = 16977;
        // N s_26_2: write-reg s_26_1 <= s_26_0
        let s_26_2: () = {
            state.write_register::<bool>(s_26_1 as isize, s_26_0);
            tracer.write_register(s_26_1 as isize, s_26_0);
        };
        // N s_26_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#8970 <= s_27_0
        fn_state.gs_8970 = s_27_0;
        // N s_27_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // C s_28_1: const #16979u : u32
        let s_28_1: u32 = 16979;
        // N s_28_2: write-reg s_28_1 <= s_28_0
        let s_28_2: () = {
            state.write_register::<bool>(s_28_1 as isize, s_28_0);
            tracer.write_register(s_28_1 as isize, s_28_0);
        };
        // N s_28_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#8969 <= s_29_0
        fn_state.gs_8969 = s_29_0;
        // N s_29_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // C s_30_1: const #16968u : u32
        let s_30_1: u32 = 16968;
        // N s_30_2: write-reg s_30_1 <= s_30_0
        let s_30_2: () = {
            state.write_register::<bool>(s_30_1 as isize, s_30_0);
            tracer.write_register(s_30_1 as isize, s_30_0);
        };
        // N s_30_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#8968 <= s_31_0
        fn_state.gs_8968 = s_31_0;
        // N s_31_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var except:struct
        let s_32_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_32_1: call AArch32_ReportHypEntry(s_32_0)
        let s_32_1: () = AArch32_ReportHypEntry(state, tracer, s_32_0);
        // N s_32_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#8966 <= s_33_0
        fn_state.gs_8966 = s_33_0;
        // N s_33_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var except:struct
        let s_34_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_34_1: call AArch32_EnterHypModeInDebugState(s_34_0)
        let s_34_1: () = AArch32_EnterHypModeInDebugState(state, tracer, s_34_0);
        // N s_34_2: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #432u : u32
        let s_35_0: u32 = 432;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call ELUsingAArch32(s_35_1)
        let s_35_2: bool = ELUsingAArch32(state, tracer, s_35_1);
        // D s_35_3: write-var gs#8960 <= s_35_2
        fn_state.gs_8960 = s_35_2;
        // N s_35_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call CurrentSecurityState(s_36_0)
        let s_36_1: u32 = CurrentSecurityState(state, tracer, s_36_0);
        // C s_36_2: const #0u : u32
        let s_36_2: u32 = 0;
        // S s_36_3: cmp-eq s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) == (s_36_2));
        // D s_36_4: write-var gs#8959 <= s_36_3
        fn_state.gs_8959 = s_36_3;
        // N s_36_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
