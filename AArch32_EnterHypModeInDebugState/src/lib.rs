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
use HSCTLR_read::*;
use AArch32_ReportHypEntry::*;
use DLR_write::*;
use u_get_HSCTLR_Type_EE::*;
use EDSCR_write::*;
use Mk_DSPSR_Type::*;
use ELR_hyp_write::*;
use u__UNKNOWN_bit::*;
use CurrentSecurityState::*;
use UpdateEDSCRFields::*;
use u__UNKNOWN_bits::*;
use SPSR_set::*;
use DSPSR_write::*;
use HaveSSBSExt::*;
use EndOfInstruction::*;
use ELUsingAArch32::*;
use AArch32_WriteMode::*;
use SynchronizeContext::*;
use u_update_EDSCR_Type_ERR::*;
use EDSCR_read::*;
use common::*;
pub fn AArch32_EnterHypModeInDebugState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    except: ProductTypeb7f99f96751e17c4,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_8918: bool,
        gs_8919: bool,
        except: ProductTypeb7f99f96751e17c4,
    }
    let fn_state = FunctionState {
        except,
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
        // N s_0_6: branch s_0_5 b9 b1
        if s_0_5 {
            return block_9(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#8918 <= s_1_0
        fn_state.gs_8918 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#8918:u8
        let s_2_0: bool = fn_state.gs_8918;
        // N s_2_1: branch s_2_0 b8 b3
        if s_2_0 {
            return block_8(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#8919 <= s_3_0
        fn_state.gs_8919 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#8919:u8
        let s_4_0: bool = fn_state.gs_8919;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var except:struct
        let s_4_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_4_3: call AArch32_ReportHypEntry(s_4_2)
        let s_4_3: () = AArch32_ReportHypEntry(state, tracer, s_4_2);
        // C s_4_4: const #400u : u32
        let s_4_4: u32 = 400;
        // D s_4_5: read-reg s_4_4:u8
        let s_4_5: u8 = {
            let value = state.read_register::<u8>(s_4_4 as isize);
            tracer.read_register(s_4_4 as isize, value);
            value
        };
        // D s_4_6: call AArch32_WriteMode(s_4_5)
        let s_4_6: () = AArch32_WriteMode(state, tracer, s_4_5);
        // C s_4_7: const #32s : i64
        let s_4_7: i64 = 32;
        // C s_4_8: cast zx s_4_7 -> i
        let s_4_8: i128 = (i128::try_from(s_4_7).unwrap());
        // S s_4_9: call __UNKNOWN_bits(s_4_8)
        let s_4_9: Bits = u__UNKNOWN_bits(state, tracer, s_4_8);
        // S s_4_10: cast reint s_4_9 -> u32
        let s_4_10: u32 = (s_4_9.value() as u32);
        // C s_4_11: const #32s : i
        let s_4_11: i128 = 32;
        // S s_4_12: cast zx s_4_10 -> bv
        let s_4_12: Bits = Bits::new(s_4_10 as u128, 32u16);
        // S s_4_13: call SPSR_set(s_4_11, s_4_12)
        let s_4_13: () = SPSR_set(state, tracer, s_4_11, s_4_12);
        // C s_4_14: const #32s : i64
        let s_4_14: i64 = 32;
        // C s_4_15: cast zx s_4_14 -> i
        let s_4_15: i128 = (i128::try_from(s_4_14).unwrap());
        // S s_4_16: call __UNKNOWN_bits(s_4_15)
        let s_4_16: Bits = u__UNKNOWN_bits(state, tracer, s_4_15);
        // S s_4_17: cast reint s_4_16 -> u32
        let s_4_17: u32 = (s_4_16.value() as u32);
        // S s_4_18: call ELR_hyp_write(s_4_17)
        let s_4_18: () = ELR_hyp_write(state, tracer, s_4_17);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: const #16993u : u32
        let s_4_20: u32 = 16993;
        // N s_4_21: write-reg s_4_20 <= s_4_19
        let s_4_21: () = {
            state.write_register::<bool>(s_4_20 as isize, s_4_19);
            tracer.write_register(s_4_20 as isize, s_4_19);
        };
        // C s_4_22: const #4s : i64
        let s_4_22: i64 = 4;
        // C s_4_23: cast zx s_4_22 -> i
        let s_4_23: i128 = (i128::try_from(s_4_22).unwrap());
        // S s_4_24: call __UNKNOWN_bits(s_4_23)
        let s_4_24: Bits = u__UNKNOWN_bits(state, tracer, s_4_23);
        // S s_4_25: cast reint s_4_24 -> u8
        let s_4_25: u8 = (s_4_24.value() as u8);
        // C s_4_26: const #3s : i
        let s_4_26: i128 = 3;
        // S s_4_27: cast zx s_4_25 -> bv
        let s_4_27: Bits = Bits::new(s_4_25 as u128, 4u16);
        // C s_4_28: const #1s : i64
        let s_4_28: i64 = 1;
        // C s_4_29: cast zx s_4_28 -> i
        let s_4_29: i128 = (i128::try_from(s_4_28).unwrap());
        // C s_4_30: const #0s : i
        let s_4_30: i128 = 0;
        // C s_4_31: add s_4_30 s_4_29
        let s_4_31: i128 = (s_4_30 + s_4_29);
        // D s_4_32: bit-extract s_4_27 s_4_26 s_4_31
        let s_4_32: Bits = (Bits::new(
            ((s_4_27) >> (s_4_26)).value(),
            u16::try_from(s_4_31).unwrap(),
        ));
        // D s_4_33: cast reint s_4_32 -> u8
        let s_4_33: bool = ((s_4_32.value()) != 0);
        // C s_4_34: const #16991u : u32
        let s_4_34: u32 = 16991;
        // N s_4_35: write-reg s_4_34 <= s_4_33
        let s_4_35: () = {
            state.write_register::<bool>(s_4_34 as isize, s_4_33);
            tracer.write_register(s_4_34 as isize, s_4_33);
        };
        // C s_4_36: const #2s : i
        let s_4_36: i128 = 2;
        // S s_4_37: cast zx s_4_25 -> bv
        let s_4_37: Bits = Bits::new(s_4_25 as u128, 4u16);
        // C s_4_38: const #1s : i64
        let s_4_38: i64 = 1;
        // C s_4_39: cast zx s_4_38 -> i
        let s_4_39: i128 = (i128::try_from(s_4_38).unwrap());
        // C s_4_40: const #0s : i
        let s_4_40: i128 = 0;
        // C s_4_41: add s_4_40 s_4_39
        let s_4_41: i128 = (s_4_40 + s_4_39);
        // D s_4_42: bit-extract s_4_37 s_4_36 s_4_41
        let s_4_42: Bits = (Bits::new(
            ((s_4_37) >> (s_4_36)).value(),
            u16::try_from(s_4_41).unwrap(),
        ));
        // D s_4_43: cast reint s_4_42 -> u8
        let s_4_43: bool = ((s_4_42.value()) != 0);
        // C s_4_44: const #16968u : u32
        let s_4_44: u32 = 16968;
        // N s_4_45: write-reg s_4_44 <= s_4_43
        let s_4_45: () = {
            state.write_register::<bool>(s_4_44 as isize, s_4_43);
            tracer.write_register(s_4_44 as isize, s_4_43);
        };
        // C s_4_46: const #1s : i
        let s_4_46: i128 = 1;
        // S s_4_47: cast zx s_4_25 -> bv
        let s_4_47: Bits = Bits::new(s_4_25 as u128, 4u16);
        // C s_4_48: const #1s : i64
        let s_4_48: i64 = 1;
        // C s_4_49: cast zx s_4_48 -> i
        let s_4_49: i128 = (i128::try_from(s_4_48).unwrap());
        // C s_4_50: const #0s : i
        let s_4_50: i128 = 0;
        // C s_4_51: add s_4_50 s_4_49
        let s_4_51: i128 = (s_4_50 + s_4_49);
        // D s_4_52: bit-extract s_4_47 s_4_46 s_4_51
        let s_4_52: Bits = (Bits::new(
            ((s_4_47) >> (s_4_46)).value(),
            u16::try_from(s_4_51).unwrap(),
        ));
        // D s_4_53: cast reint s_4_52 -> u8
        let s_4_53: bool = ((s_4_52.value()) != 0);
        // C s_4_54: const #16979u : u32
        let s_4_54: u32 = 16979;
        // N s_4_55: write-reg s_4_54 <= s_4_53
        let s_4_55: () = {
            state.write_register::<bool>(s_4_54 as isize, s_4_53);
            tracer.write_register(s_4_54 as isize, s_4_53);
        };
        // C s_4_56: const #0s : i
        let s_4_56: i128 = 0;
        // S s_4_57: cast zx s_4_25 -> bv
        let s_4_57: Bits = Bits::new(s_4_25 as u128, 4u16);
        // C s_4_58: const #1s : i64
        let s_4_58: i64 = 1;
        // C s_4_59: cast zx s_4_58 -> i
        let s_4_59: i128 = (i128::try_from(s_4_58).unwrap());
        // C s_4_60: const #0s : i
        let s_4_60: i128 = 0;
        // C s_4_61: add s_4_60 s_4_59
        let s_4_61: i128 = (s_4_60 + s_4_59);
        // D s_4_62: bit-extract s_4_57 s_4_56 s_4_61
        let s_4_62: Bits = (Bits::new(
            ((s_4_57) >> (s_4_56)).value(),
            u16::try_from(s_4_61).unwrap(),
        ));
        // D s_4_63: cast reint s_4_62 -> u8
        let s_4_63: bool = ((s_4_62.value()) != 0);
        // C s_4_64: const #16977u : u32
        let s_4_64: u32 = 16977;
        // N s_4_65: write-reg s_4_64 <= s_4_63
        let s_4_65: () = {
            state.write_register::<bool>(s_4_64 as isize, s_4_63);
            tracer.write_register(s_4_64 as isize, s_4_63);
        };
        // C s_4_66: const #32s : i64
        let s_4_66: i64 = 32;
        // C s_4_67: cast zx s_4_66 -> i
        let s_4_67: i128 = (i128::try_from(s_4_66).unwrap());
        // S s_4_68: call __UNKNOWN_bits(s_4_67)
        let s_4_68: Bits = u__UNKNOWN_bits(state, tracer, s_4_67);
        // S s_4_69: cast reint s_4_68 -> u32
        let s_4_69: u32 = (s_4_68.value() as u32);
        // S s_4_70: call DLR_write(s_4_69)
        let s_4_70: () = DLR_write(state, tracer, s_4_69);
        // C s_4_71: const #32s : i64
        let s_4_71: i64 = 32;
        // C s_4_72: cast zx s_4_71 -> i
        let s_4_72: i128 = (i128::try_from(s_4_71).unwrap());
        // S s_4_73: call __UNKNOWN_bits(s_4_72)
        let s_4_73: Bits = u__UNKNOWN_bits(state, tracer, s_4_72);
        // S s_4_74: cast reint s_4_73 -> u32
        let s_4_74: u32 = (s_4_73.value() as u32);
        // S s_4_75: call Mk_DSPSR_Type(s_4_74)
        let s_4_75: ProductType700c18a878c5601b = Mk_DSPSR_Type(state, tracer, s_4_74);
        // S s_4_76: call DSPSR_write(s_4_75)
        let s_4_76: () = DSPSR_write(state, tracer, s_4_75);
        // C s_4_77: const #() : ()
        let s_4_77: () = ();
        // S s_4_78: call HSCTLR_read(s_4_77)
        let s_4_78: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_4_77);
        // S s_4_79: call _get_HSCTLR_Type_EE(s_4_78)
        let s_4_79: bool = u_get_HSCTLR_Type_EE(state, tracer, s_4_78);
        // C s_4_80: const #16974u : u32
        let s_4_80: u32 = 16974;
        // N s_4_81: write-reg s_4_80 <= s_4_79
        let s_4_81: () = {
            state.write_register::<bool>(s_4_80 as isize, s_4_79);
            tracer.write_register(s_4_80 as isize, s_4_79);
        };
        // C s_4_82: const #0u : u8
        let s_4_82: bool = false;
        // C s_4_83: const #16980u : u32
        let s_4_83: u32 = 16980;
        // N s_4_84: write-reg s_4_83 <= s_4_82
        let s_4_84: () = {
            state.write_register::<bool>(s_4_83 as isize, s_4_82);
            tracer.write_register(s_4_83 as isize, s_4_82);
        };
        // C s_4_85: const #0u : u8
        let s_4_85: u8 = 0;
        // C s_4_86: const #16981u : u32
        let s_4_86: u32 = 16981;
        // N s_4_87: write-reg s_4_86 <= s_4_85
        let s_4_87: () = {
            state.write_register::<u8>(s_4_86 as isize, s_4_85);
            tracer.write_register(s_4_86 as isize, s_4_85);
        };
        // C s_4_88: const #() : ()
        let s_4_88: () = ();
        // S s_4_89: call HaveSSBSExt(s_4_88)
        let s_4_89: bool = HaveSSBSExt(state, tracer, s_4_88);
        // N s_4_90: branch s_4_89 b7 b5
        if s_4_89 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EDSCR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_6_0);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // S s_6_3: call _update_EDSCR_Type_ERR(s_6_1, s_6_2)
        let s_6_3: ProductType700c18a878c5601b = u_update_EDSCR_Type_ERR(
            state,
            tracer,
            s_6_1,
            s_6_2,
        );
        // S s_6_4: call EDSCR_write(s_6_3)
        let s_6_4: () = EDSCR_write(state, tracer, s_6_3);
        // C s_6_5: const #() : ()
        let s_6_5: () = ();
        // S s_6_6: call UpdateEDSCRFields(s_6_5)
        let s_6_6: () = UpdateEDSCRFields(state, tracer, s_6_5);
        // C s_6_7: const #() : ()
        let s_6_7: () = ();
        // S s_6_8: call EndOfInstruction(s_6_7)
        let s_6_8: () = EndOfInstruction(state, tracer, s_6_7);
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call __UNKNOWN_bit(s_7_0)
        let s_7_1: bool = u__UNKNOWN_bit(state, tracer, s_7_0);
        // C s_7_2: const #16992u : u32
        let s_7_2: u32 = 16992;
        // N s_7_3: write-reg s_7_2 <= s_7_1
        let s_7_3: () = {
            state.write_register::<bool>(s_7_2 as isize, s_7_1);
            tracer.write_register(s_7_2 as isize, s_7_1);
        };
        // N s_7_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call ELUsingAArch32(s_8_1)
        let s_8_2: bool = ELUsingAArch32(state, tracer, s_8_1);
        // D s_8_3: write-var gs#8919 <= s_8_2
        fn_state.gs_8919 = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CurrentSecurityState(s_9_0)
        let s_9_1: u32 = CurrentSecurityState(state, tracer, s_9_0);
        // C s_9_2: const #0u : u32
        let s_9_2: u32 = 0;
        // S s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // D s_9_4: write-var gs#8918 <= s_9_3
        fn_state.gs_8918 = s_9_3;
        // N s_9_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
