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
use u_get_HCR_Type_AMO::*;
use AArch64_vESBOperation::*;
use Halted::*;
use HCR_read::*;
use TTBCR_read::*;
use ExternalDebugInterruptsDisabled::*;
use Zeros::*;
use u_get_HCR_Type_VA::*;
use u_get_HCR_Type_TGE::*;
use u_update_HCR_Type_VA::*;
use HCR_write::*;
use ELUsingAArch32::*;
use Bit::*;
use u_get_TTBCR_Type_EAE::*;
use Mk_VDISR_Type::*;
use VDISR_write::*;
use EL2Enabled::*;
use VDFSR_read::*;
use common::*;
pub fn AArch32_vESBOperation<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31702: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_24719: ProductType700c18a878c5601b,
        gs_31709: bool,
        vSEI_pending: bool,
        gs_31710: bool,
        gs_31712: bool,
        ga_24716: ProductType700c18a878c5601b,
        gs_31708: bool,
        vmasked: bool,
        gs_31704: bool,
        gs_31711: bool,
        gs_31703: bool,
        syndrome: u32,
        gs_31702: (),
    }
    let fn_state = FunctionState {
        gs_31702,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b28 b1
        if s_0_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#31703 <= s_1_6
        fn_state.gs_31703 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31703:u8
        let s_2_0: bool = fn_state.gs_31703;
        // N s_2_1: branch s_2_0 b27 b3
        if s_2_0 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#31704 <= s_3_0
        fn_state.gs_31704 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31704:u8
        let s_4_0: bool = fn_state.gs_31704;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #432u : u32
        let s_4_2: u32 = 432;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: call ELUsingAArch32(s_4_3)
        let s_4_4: bool = ELUsingAArch32(state, tracer, s_4_3);
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b26 b5
        if s_4_5 {
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_5_0);
        // S s_5_2: call _get_HCR_Type_TGE(s_5_1)
        let s_5_2: bool = u_get_HCR_Type_TGE(state, tracer, s_5_1);
        // S s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // C s_5_4: const #0u : u8
        let s_5_4: bool = false;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 1u16);
        // S s_5_6: cmp-eq s_5_3 s_5_5
        let s_5_6: bool = ((s_5_3) == (s_5_5));
        // N s_5_7: branch s_5_6 b25 b6
        if s_5_6 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#31708 <= s_6_0
        fn_state.gs_31708 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#31708:u8
        let s_7_0: bool = fn_state.gs_31708;
        // N s_7_1: branch s_7_0 b24 b8
        if s_7_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#31709 <= s_8_0
        fn_state.gs_31709 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#31709:u8
        let s_9_0: bool = fn_state.gs_31709;
        // D s_9_1: write-var vSEI_pending <= s_9_0
        fn_state.vSEI_pending = s_9_0;
        // C s_9_2: const #() : ()
        let s_9_2: () = ();
        // S s_9_3: call Halted(s_9_2)
        let s_9_3: bool = Halted(state, tracer, s_9_2);
        // N s_9_4: branch s_9_3 b23 b10
        if s_9_3 {
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
        // C s_10_0: const #440u : u32
        let s_10_0: u32 = 440;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call ExternalDebugInterruptsDisabled(s_10_1)
        let s_10_2: bool = ExternalDebugInterruptsDisabled(state, tracer, s_10_1);
        // D s_10_3: write-var gs#31710 <= s_10_2
        fn_state.gs_31710 = s_10_2;
        // N s_10_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#31710:u8
        let s_11_0: bool = fn_state.gs_31710;
        // N s_11_1: branch s_11_0 b22 b12
        if s_11_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16968u : u32
        let s_12_0: u32 = 16968;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: bool = {
            let value = state.read_register::<bool>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 1u16);
        // C s_12_3: const #1u : u8
        let s_12_3: bool = true;
        // C s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 1u16);
        // D s_12_5: cmp-eq s_12_2 s_12_4
        let s_12_5: bool = ((s_12_2) == (s_12_4));
        // D s_12_6: write-var gs#31711 <= s_12_5
        fn_state.gs_31711 = s_12_5;
        // N s_12_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#31711:u8
        let s_13_0: bool = fn_state.gs_31711;
        // D s_13_1: write-var vmasked <= s_13_0
        fn_state.vmasked = s_13_0;
        // D s_13_2: read-var vSEI_pending:u8
        let s_13_2: bool = fn_state.vSEI_pending;
        // N s_13_3: branch s_13_2 b21 b14
        if s_13_2 {
            return block_21(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#31712 <= s_14_0
        fn_state.gs_31712 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#31712:u8
        let s_15_0: bool = fn_state.gs_31712;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #32s : i
        let s_17_0: i128 = 32;
        // S s_17_1: call Zeros(s_17_0)
        let s_17_1: Bits = Zeros(state, tracer, s_17_0);
        // S s_17_2: cast reint s_17_1 -> u32
        let s_17_2: u32 = (s_17_1.value() as u32);
        // D s_17_3: write-var syndrome <= s_17_2
        fn_state.syndrome = s_17_2;
        // C s_17_4: const #1u : u8
        let s_17_4: bool = true;
        // S s_17_5: call Bit(s_17_4)
        let s_17_5: bool = Bit(state, tracer, s_17_4);
        // C s_17_6: const #31s : i
        let s_17_6: i128 = 31;
        // D s_17_7: read-var syndrome:u32
        let s_17_7: u32 = fn_state.syndrome;
        // D s_17_8: cast zx s_17_7 -> bv
        let s_17_8: Bits = Bits::new(s_17_7 as u128, 32u16);
        // C s_17_9: const #1u : u64
        let s_17_9: u64 = 1;
        // D s_17_10: bit-insert s_17_8 s_17_8 s_17_6 s_17_9
        let s_17_10: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_9 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_8.length(),
            );
            (s_17_8 & mask) | (s_17_8 << s_17_6)
        };
        // D s_17_11: cast reint s_17_10 -> u32
        let s_17_11: u32 = (s_17_10.value() as u32);
        // D s_17_12: write-var syndrome <= s_17_11
        fn_state.syndrome = s_17_11;
        // C s_17_13: const #() : ()
        let s_17_13: () = ();
        // S s_17_14: call VDFSR_read(s_17_13)
        let s_17_14: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_17_13);
        // D s_17_15: write-var ga#24716 <= s_17_14
        fn_state.ga_24716 = s_17_14;
        // D s_17_16: read-var ga#24716.0:struct
        let s_17_16: u32 = fn_state.ga_24716._0;
        // C s_17_17: const #14s : i
        let s_17_17: i128 = 14;
        // D s_17_18: cast zx s_17_16 -> bv
        let s_17_18: Bits = Bits::new(s_17_16 as u128, 32u16);
        // C s_17_19: const #1s : i64
        let s_17_19: i64 = 1;
        // C s_17_20: cast zx s_17_19 -> i
        let s_17_20: i128 = (i128::try_from(s_17_19).unwrap());
        // C s_17_21: const #1s : i
        let s_17_21: i128 = 1;
        // C s_17_22: add s_17_21 s_17_20
        let s_17_22: i128 = (s_17_21 + s_17_20);
        // D s_17_23: bit-extract s_17_18 s_17_17 s_17_22
        let s_17_23: Bits = (Bits::new(
            ((s_17_18) >> (s_17_17)).value(),
            u16::try_from(s_17_22).unwrap(),
        ));
        // D s_17_24: cast reint s_17_23 -> u8
        let s_17_24: u8 = (s_17_23.value() as u8);
        // C s_17_25: const #14s : i
        let s_17_25: i128 = 14;
        // D s_17_26: read-var syndrome:u32
        let s_17_26: u32 = fn_state.syndrome;
        // D s_17_27: cast zx s_17_26 -> bv
        let s_17_27: Bits = Bits::new(s_17_26 as u128, 32u16);
        // D s_17_28: cast zx s_17_24 -> bv
        let s_17_28: Bits = Bits::new(s_17_24 as u128, 2u16);
        // C s_17_29: const #1s : i
        let s_17_29: i128 = 1;
        // C s_17_30: const #1u : u64
        let s_17_30: u64 = 1;
        // C s_17_31: cast zx s_17_30 -> bv
        let s_17_31: Bits = Bits::new(s_17_30 as u128, 64u16);
        // C s_17_32: lsl s_17_31 s_17_29
        let s_17_32: Bits = s_17_31 << s_17_29;
        // C s_17_33: sub s_17_32 s_17_31
        let s_17_33: Bits = ((s_17_32) - (s_17_31));
        // D s_17_34: and s_17_28 s_17_33
        let s_17_34: Bits = ((s_17_28) & (s_17_33));
        // D s_17_35: lsl s_17_34 s_17_25
        let s_17_35: Bits = s_17_34 << s_17_25;
        // C s_17_36: lsl s_17_33 s_17_25
        let s_17_36: Bits = s_17_33 << s_17_25;
        // C s_17_37: cmpl s_17_36
        let s_17_37: Bits = !s_17_36;
        // D s_17_38: and s_17_27 s_17_37
        let s_17_38: Bits = ((s_17_27) & (s_17_37));
        // D s_17_39: or s_17_38 s_17_35
        let s_17_39: Bits = ((s_17_38) | (s_17_35));
        // D s_17_40: cast reint s_17_39 -> u32
        let s_17_40: u32 = (s_17_39.value() as u32);
        // D s_17_41: write-var syndrome <= s_17_40
        fn_state.syndrome = s_17_40;
        // C s_17_42: const #() : ()
        let s_17_42: () = ();
        // S s_17_43: call VDFSR_read(s_17_42)
        let s_17_43: ProductType700c18a878c5601b = VDFSR_read(state, tracer, s_17_42);
        // D s_17_44: write-var ga#24719 <= s_17_43
        fn_state.ga_24719 = s_17_43;
        // D s_17_45: read-var ga#24719.0:struct
        let s_17_45: u32 = fn_state.ga_24719._0;
        // C s_17_46: const #12s : i
        let s_17_46: i128 = 12;
        // D s_17_47: cast zx s_17_45 -> bv
        let s_17_47: Bits = Bits::new(s_17_45 as u128, 32u16);
        // C s_17_48: const #1u : u64
        let s_17_48: u64 = 1;
        // D s_17_49: bit-extract s_17_47 s_17_46 s_17_48
        let s_17_49: Bits = (Bits::new(
            ((s_17_47) >> (s_17_46)).value(),
            u16::try_from(s_17_48).unwrap(),
        ));
        // D s_17_50: cast reint s_17_49 -> u8
        let s_17_50: bool = ((s_17_49.value()) != 0);
        // C s_17_51: const #0s : i
        let s_17_51: i128 = 0;
        // C s_17_52: const #0u : u64
        let s_17_52: u64 = 0;
        // D s_17_53: cast zx s_17_50 -> u64
        let s_17_53: u64 = (s_17_50 as u64);
        // C s_17_54: const #1u : u64
        let s_17_54: u64 = 1;
        // D s_17_55: and s_17_53 s_17_54
        let s_17_55: u64 = ((s_17_53) & (s_17_54));
        // D s_17_56: cmp-eq s_17_55 s_17_54
        let s_17_56: bool = ((s_17_55) == (s_17_54));
        // D s_17_57: lsl s_17_53 s_17_51
        let s_17_57: u64 = s_17_53 << s_17_51;
        // D s_17_58: or s_17_52 s_17_57
        let s_17_58: u64 = ((s_17_52) | (s_17_57));
        // D s_17_59: cmpl s_17_57
        let s_17_59: u64 = !s_17_57;
        // D s_17_60: and s_17_52 s_17_59
        let s_17_60: u64 = ((s_17_52) & (s_17_59));
        // D s_17_61: select s_17_56 s_17_58 s_17_60
        let s_17_61: u64 = if s_17_56 { s_17_58 } else { s_17_60 };
        // D s_17_62: cast trunc s_17_61 -> u8
        let s_17_62: bool = ((s_17_61) != 0);
        // D s_17_63: call Bit(s_17_62)
        let s_17_63: bool = Bit(state, tracer, s_17_62);
        // C s_17_64: const #12s : i
        let s_17_64: i128 = 12;
        // D s_17_65: read-var syndrome:u32
        let s_17_65: u32 = fn_state.syndrome;
        // D s_17_66: cast zx s_17_65 -> bv
        let s_17_66: Bits = Bits::new(s_17_65 as u128, 32u16);
        // C s_17_67: const #1u : u64
        let s_17_67: u64 = 1;
        // D s_17_68: bit-insert s_17_66 s_17_66 s_17_64 s_17_67
        let s_17_68: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_67 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_66.length(),
            );
            (s_17_66 & mask) | (s_17_66 << s_17_64)
        };
        // D s_17_69: cast reint s_17_68 -> u32
        let s_17_69: u32 = (s_17_68.value() as u32);
        // D s_17_70: write-var syndrome <= s_17_69
        fn_state.syndrome = s_17_69;
        // C s_17_71: const #() : ()
        let s_17_71: () = ();
        // S s_17_72: call TTBCR_read(s_17_71)
        let s_17_72: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_17_71);
        // S s_17_73: call _get_TTBCR_Type_EAE(s_17_72)
        let s_17_73: bool = u_get_TTBCR_Type_EAE(state, tracer, s_17_72);
        // S s_17_74: call Bit(s_17_73)
        let s_17_74: bool = Bit(state, tracer, s_17_73);
        // C s_17_75: const #9s : i
        let s_17_75: i128 = 9;
        // D s_17_76: read-var syndrome:u32
        let s_17_76: u32 = fn_state.syndrome;
        // D s_17_77: cast zx s_17_76 -> bv
        let s_17_77: Bits = Bits::new(s_17_76 as u128, 32u16);
        // C s_17_78: const #1u : u64
        let s_17_78: u64 = 1;
        // D s_17_79: bit-insert s_17_77 s_17_77 s_17_75 s_17_78
        let s_17_79: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_78 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_77.length(),
            );
            (s_17_77 & mask) | (s_17_77 << s_17_75)
        };
        // D s_17_80: cast reint s_17_79 -> u32
        let s_17_80: u32 = (s_17_79.value() as u32);
        // D s_17_81: write-var syndrome <= s_17_80
        fn_state.syndrome = s_17_80;
        // C s_17_82: const #() : ()
        let s_17_82: () = ();
        // S s_17_83: call TTBCR_read(s_17_82)
        let s_17_83: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_17_82);
        // S s_17_84: call _get_TTBCR_Type_EAE(s_17_83)
        let s_17_84: bool = u_get_TTBCR_Type_EAE(state, tracer, s_17_83);
        // S s_17_85: cast zx s_17_84 -> bv
        let s_17_85: Bits = Bits::new(s_17_84 as u128, 1u16);
        // C s_17_86: const #1u : u8
        let s_17_86: bool = true;
        // C s_17_87: cast zx s_17_86 -> bv
        let s_17_87: Bits = Bits::new(s_17_86 as u128, 1u16);
        // S s_17_88: cmp-eq s_17_85 s_17_87
        let s_17_88: bool = ((s_17_85) == (s_17_87));
        // N s_17_89: branch s_17_88 b20 b18
        if s_17_88 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #22u : u8
        let s_18_0: u8 = 22;
        // C s_18_1: const #4s : i
        let s_18_1: i128 = 4;
        // C s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 5u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #0s : i
        let s_18_5: i128 = 0;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_1 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_1)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: bool = ((s_18_7.value()) != 0);
        // C s_18_9: const #10s : i
        let s_18_9: i128 = 10;
        // D s_18_10: read-var syndrome:u32
        let s_18_10: u32 = fn_state.syndrome;
        // D s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 32u16);
        // D s_18_12: cast zx s_18_8 -> bv
        let s_18_12: Bits = Bits::new(s_18_8 as u128, 1u16);
        // C s_18_13: const #0s : i
        let s_18_13: i128 = 0;
        // C s_18_14: const #1u : u64
        let s_18_14: u64 = 1;
        // C s_18_15: cast zx s_18_14 -> bv
        let s_18_15: Bits = Bits::new(s_18_14 as u128, 64u16);
        // C s_18_16: lsl s_18_15 s_18_13
        let s_18_16: Bits = s_18_15 << s_18_13;
        // C s_18_17: sub s_18_16 s_18_15
        let s_18_17: Bits = ((s_18_16) - (s_18_15));
        // D s_18_18: and s_18_12 s_18_17
        let s_18_18: Bits = ((s_18_12) & (s_18_17));
        // D s_18_19: lsl s_18_18 s_18_9
        let s_18_19: Bits = s_18_18 << s_18_9;
        // C s_18_20: lsl s_18_17 s_18_9
        let s_18_20: Bits = s_18_17 << s_18_9;
        // C s_18_21: cmpl s_18_20
        let s_18_21: Bits = !s_18_20;
        // D s_18_22: and s_18_11 s_18_21
        let s_18_22: Bits = ((s_18_11) & (s_18_21));
        // D s_18_23: or s_18_22 s_18_19
        let s_18_23: Bits = ((s_18_22) | (s_18_19));
        // D s_18_24: cast reint s_18_23 -> u32
        let s_18_24: u32 = (s_18_23.value() as u32);
        // D s_18_25: write-var syndrome <= s_18_24
        fn_state.syndrome = s_18_24;
        // C s_18_26: const #0s : i
        let s_18_26: i128 = 0;
        // C s_18_27: cast zx s_18_0 -> bv
        let s_18_27: Bits = Bits::new(s_18_0 as u128, 5u16);
        // C s_18_28: const #1s : i64
        let s_18_28: i64 = 1;
        // C s_18_29: cast zx s_18_28 -> i
        let s_18_29: i128 = (i128::try_from(s_18_28).unwrap());
        // C s_18_30: const #3s : i
        let s_18_30: i128 = 3;
        // C s_18_31: add s_18_30 s_18_29
        let s_18_31: i128 = (s_18_30 + s_18_29);
        // D s_18_32: bit-extract s_18_27 s_18_26 s_18_31
        let s_18_32: Bits = (Bits::new(
            ((s_18_27) >> (s_18_26)).value(),
            u16::try_from(s_18_31).unwrap(),
        ));
        // D s_18_33: cast reint s_18_32 -> u8
        let s_18_33: u8 = (s_18_32.value() as u8);
        // C s_18_34: const #0s : i
        let s_18_34: i128 = 0;
        // D s_18_35: read-var syndrome:u32
        let s_18_35: u32 = fn_state.syndrome;
        // D s_18_36: cast zx s_18_35 -> bv
        let s_18_36: Bits = Bits::new(s_18_35 as u128, 32u16);
        // D s_18_37: cast zx s_18_33 -> bv
        let s_18_37: Bits = Bits::new(s_18_33 as u128, 4u16);
        // C s_18_38: const #3s : i
        let s_18_38: i128 = 3;
        // C s_18_39: const #1u : u64
        let s_18_39: u64 = 1;
        // C s_18_40: cast zx s_18_39 -> bv
        let s_18_40: Bits = Bits::new(s_18_39 as u128, 64u16);
        // C s_18_41: lsl s_18_40 s_18_38
        let s_18_41: Bits = s_18_40 << s_18_38;
        // C s_18_42: sub s_18_41 s_18_40
        let s_18_42: Bits = ((s_18_41) - (s_18_40));
        // D s_18_43: and s_18_37 s_18_42
        let s_18_43: Bits = ((s_18_37) & (s_18_42));
        // D s_18_44: lsl s_18_43 s_18_34
        let s_18_44: Bits = s_18_43 << s_18_34;
        // C s_18_45: lsl s_18_42 s_18_34
        let s_18_45: Bits = s_18_42 << s_18_34;
        // C s_18_46: cmpl s_18_45
        let s_18_46: Bits = !s_18_45;
        // D s_18_47: and s_18_36 s_18_46
        let s_18_47: Bits = ((s_18_36) & (s_18_46));
        // D s_18_48: or s_18_47 s_18_44
        let s_18_48: Bits = ((s_18_47) | (s_18_44));
        // D s_18_49: cast reint s_18_48 -> u32
        let s_18_49: u32 = (s_18_48.value() as u32);
        // D s_18_50: write-var syndrome <= s_18_49
        fn_state.syndrome = s_18_49;
        // N s_18_51: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var syndrome:u32
        let s_19_0: u32 = fn_state.syndrome;
        // D s_19_1: call Mk_VDISR_Type(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = Mk_VDISR_Type(state, tracer, s_19_0);
        // D s_19_2: call VDISR_write(s_19_1)
        let s_19_2: () = VDISR_write(state, tracer, s_19_1);
        // C s_19_3: const #() : ()
        let s_19_3: () = ();
        // S s_19_4: call HCR_read(s_19_3)
        let s_19_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_19_3);
        // C s_19_5: const #0u : u8
        let s_19_5: bool = false;
        // S s_19_6: call _update_HCR_Type_VA(s_19_4, s_19_5)
        let s_19_6: ProductType700c18a878c5601b = u_update_HCR_Type_VA(
            state,
            tracer,
            s_19_4,
            s_19_5,
        );
        // S s_19_7: call HCR_write(s_19_6)
        let s_19_7: () = HCR_write(state, tracer, s_19_6);
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0s : i
        let s_20_0: i128 = 0;
        // D s_20_1: read-var syndrome:u32
        let s_20_1: u32 = fn_state.syndrome;
        // D s_20_2: cast zx s_20_1 -> bv
        let s_20_2: Bits = Bits::new(s_20_1 as u128, 32u16);
        // C s_20_3: const #17u : u8
        let s_20_3: u8 = 17;
        // C s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 6u16);
        // C s_20_5: const #5s : i
        let s_20_5: i128 = 5;
        // C s_20_6: const #1u : u64
        let s_20_6: u64 = 1;
        // C s_20_7: cast zx s_20_6 -> bv
        let s_20_7: Bits = Bits::new(s_20_6 as u128, 64u16);
        // C s_20_8: lsl s_20_7 s_20_5
        let s_20_8: Bits = s_20_7 << s_20_5;
        // C s_20_9: sub s_20_8 s_20_7
        let s_20_9: Bits = ((s_20_8) - (s_20_7));
        // C s_20_10: and s_20_4 s_20_9
        let s_20_10: Bits = ((s_20_4) & (s_20_9));
        // C s_20_11: lsl s_20_10 s_20_0
        let s_20_11: Bits = s_20_10 << s_20_0;
        // C s_20_12: lsl s_20_9 s_20_0
        let s_20_12: Bits = s_20_9 << s_20_0;
        // C s_20_13: cmpl s_20_12
        let s_20_13: Bits = !s_20_12;
        // D s_20_14: and s_20_2 s_20_13
        let s_20_14: Bits = ((s_20_2) & (s_20_13));
        // D s_20_15: or s_20_14 s_20_11
        let s_20_15: Bits = ((s_20_14) | (s_20_11));
        // D s_20_16: cast reint s_20_15 -> u32
        let s_20_16: u32 = (s_20_15.value() as u32);
        // D s_20_17: write-var syndrome <= s_20_16
        fn_state.syndrome = s_20_16;
        // N s_20_18: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var vmasked:u8
        let s_21_0: bool = fn_state.vmasked;
        // D s_21_1: write-var gs#31712 <= s_21_0
        fn_state.gs_31712 = s_21_0;
        // N s_21_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#31711 <= s_22_0
        fn_state.gs_31711 = s_22_0;
        // N s_22_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#31710 <= s_23_0
        fn_state.gs_31710 = s_23_0;
        // N s_23_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_HCR_Type_VA(s_24_1)
        let s_24_2: bool = u_get_HCR_Type_VA(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#31709 <= s_24_6
        fn_state.gs_31709 = s_24_6;
        // N s_24_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call HCR_read(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_25_0);
        // S s_25_2: call _get_HCR_Type_AMO(s_25_1)
        let s_25_2: bool = u_get_HCR_Type_AMO(state, tracer, s_25_1);
        // S s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // C s_25_4: const #1u : u8
        let s_25_4: bool = true;
        // C s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 1u16);
        // S s_25_6: cmp-eq s_25_3 s_25_5
        let s_25_6: bool = ((s_25_3) == (s_25_5));
        // D s_25_7: write-var gs#31708 <= s_25_6
        fn_state.gs_31708 = s_25_6;
        // N s_25_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call AArch64_vESBOperation(s_26_0)
        let s_26_1: () = AArch64_vESBOperation(state, tracer, s_26_0);
        // N s_26_2: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // D s_27_2: write-var gs#31704 <= s_27_1
        fn_state.gs_31704 = s_27_1;
        // N s_27_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#31703 <= s_28_0
        fn_state.gs_31703 = s_28_0;
        // N s_28_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
