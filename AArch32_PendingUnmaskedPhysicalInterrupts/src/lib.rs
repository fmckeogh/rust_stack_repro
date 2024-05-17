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
use u_get_SCR_Type_IRQ::*;
use u_get_SCR_Type_FIQ::*;
use HCR_read::*;
use IRQPending::*;
use u_get_SCR_Type_FW::*;
use CurrentSecurityState::*;
use u_get_HCR_Type_TGE::*;
use AArch64_PendingUnmaskedPhysicalInterrupts::*;
use u_get_SCR_Type_AW::*;
use ELUsingAArch32::*;
use u_get_HCR_Type_FMO::*;
use IsPhysicalSErrorPending::*;
use Bit::*;
use EffectiveEA::*;
use u_get_HCR_Type_IMO::*;
use FIQPending::*;
use EL2Enabled::*;
use u_get_HCR_Type_AMO::*;
use common::*;
pub fn AArch32_PendingUnmaskedPhysicalInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_327497: (),
) -> ProductTyped8f896a024a4e2cb {
    #[derive(Default)]
    struct FunctionState {
        gs_327513: bool,
        se_pending: bool,
        gs_327507: bool,
        gs_327515: bool,
        pending: u8,
        ga_367699: ProductType8b847afc727d5818,
        mask_overrideshadow_7999: u8,
        gs_327506: bool,
        mask: u8,
        irq_pending: bool,
        return_value: ProductTyped8f896a024a4e2cb,
        gs_327498: bool,
        gs_327508: bool,
        gs_327511: bool,
        fiq: bool,
        fiq_pending: bool,
        ga_367698: ProductType8b847afc727d5818,
        gs_327512: bool,
        gs_327514: bool,
        gs_327497: (),
    }
    let fn_state = FunctionState {
        gs_327497,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b60 b1
        if s_0_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#327498 <= s_1_0
        fn_state.gs_327498 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_2_0: read-var gs#327498:u8
        let s_2_0: bool = fn_state.gs_327498;
        // N s_2_1: branch s_2_0 b59 b3
        if s_2_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call IsPhysicalSErrorPending(s_3_0)
        let s_3_1: bool = IsPhysicalSErrorPending(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b58 b4
        if s_3_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var se_pending <= s_4_0
        fn_state.se_pending = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call IRQPending(s_5_0)
        let s_5_1: ProductType8b847afc727d5818 = IRQPending(state, tracer, s_5_0);
        // D s_5_2: write-var ga#367698 <= s_5_1
        fn_state.ga_367698 = s_5_1;
        // D s_5_3: read-var ga#367698.0:struct
        let s_5_3: bool = fn_state.ga_367698._0;
        // C s_5_4: const #() : ()
        let s_5_4: () = ();
        // S s_5_5: call FIQPending(s_5_4)
        let s_5_5: ProductType8b847afc727d5818 = FIQPending(state, tracer, s_5_4);
        // D s_5_6: write-var ga#367699 <= s_5_5
        fn_state.ga_367699 = s_5_5;
        // D s_5_7: read-var ga#367699.0:struct
        let s_5_7: bool = fn_state.ga_367699._0;
        // D s_5_8: write-var fiq <= s_5_7
        fn_state.fiq = s_5_7;
        // N s_5_9: branch s_5_3 b57 b6
        if s_5_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var irq_pending <= s_6_0
        fn_state.irq_pending = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_7_0: read-var fiq:u8
        let s_7_0: bool = fn_state.fiq;
        // N s_7_1: branch s_7_0 b56 b8
        if s_7_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var fiq_pending <= s_8_0
        fn_state.fiq_pending = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_9_0: read-var se_pending:u8
        let s_9_0: bool = fn_state.se_pending;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // D s_9_2: read-var irq_pending:u8
        let s_9_2: bool = fn_state.irq_pending;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cast reint s_9_1 -> u128
        let s_9_4: u128 = (s_9_1.value() as u128);
        // D s_9_5: size-of s_9_1
        let s_9_5: u16 = s_9_1.length();
        // D s_9_6: cast reint s_9_3 -> u128
        let s_9_6: u128 = (s_9_3.value() as u128);
        // D s_9_7: size-of s_9_3
        let s_9_7: u16 = s_9_3.length();
        // D s_9_8: lsl s_9_4 s_9_7
        let s_9_8: u128 = s_9_4 << s_9_7;
        // D s_9_9: or s_9_8 s_9_6
        let s_9_9: u128 = ((s_9_8) | (s_9_6));
        // D s_9_10: add s_9_5 s_9_7
        let s_9_10: u16 = (s_9_5 + s_9_7);
        // D s_9_11: create-bits s_9_9 s_9_10
        let s_9_11: Bits = Bits::new(s_9_9, s_9_10);
        // D s_9_12: cast reint s_9_11 -> u8
        let s_9_12: u8 = (s_9_11.value() as u8);
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 2u16);
        // D s_9_14: read-var fiq_pending:u8
        let s_9_14: bool = fn_state.fiq_pending;
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 1u16);
        // D s_9_16: cast reint s_9_13 -> u128
        let s_9_16: u128 = (s_9_13.value() as u128);
        // D s_9_17: size-of s_9_13
        let s_9_17: u16 = s_9_13.length();
        // D s_9_18: cast reint s_9_15 -> u128
        let s_9_18: u128 = (s_9_15.value() as u128);
        // D s_9_19: size-of s_9_15
        let s_9_19: u16 = s_9_15.length();
        // D s_9_20: lsl s_9_16 s_9_19
        let s_9_20: u128 = s_9_16 << s_9_19;
        // D s_9_21: or s_9_20 s_9_18
        let s_9_21: u128 = ((s_9_20) | (s_9_18));
        // D s_9_22: add s_9_17 s_9_19
        let s_9_22: u16 = (s_9_17 + s_9_19);
        // D s_9_23: create-bits s_9_21 s_9_22
        let s_9_23: Bits = Bits::new(s_9_21, s_9_22);
        // D s_9_24: cast reint s_9_23 -> u8
        let s_9_24: u8 = (s_9_23.value() as u8);
        // D s_9_25: write-var pending <= s_9_24
        fn_state.pending = s_9_24;
        // C s_9_26: const #16968u : u32
        let s_9_26: u32 = 16968;
        // D s_9_27: read-reg s_9_26:u8
        let s_9_27: bool = {
            let value = state.read_register::<bool>(s_9_26 as isize);
            tracer.read_register(s_9_26 as isize, value);
            value
        };
        // C s_9_28: const #16979u : u32
        let s_9_28: u32 = 16979;
        // D s_9_29: read-reg s_9_28:u8
        let s_9_29: bool = {
            let value = state.read_register::<bool>(s_9_28 as isize);
            tracer.read_register(s_9_28 as isize, value);
            value
        };
        // C s_9_30: const #16977u : u32
        let s_9_30: u32 = 16977;
        // D s_9_31: read-reg s_9_30:u8
        let s_9_31: bool = {
            let value = state.read_register::<bool>(s_9_30 as isize);
            tracer.read_register(s_9_30 as isize, value);
            value
        };
        // D s_9_32: cast zx s_9_29 -> bv
        let s_9_32: Bits = Bits::new(s_9_29 as u128, 1u16);
        // D s_9_33: cast zx s_9_31 -> bv
        let s_9_33: Bits = Bits::new(s_9_31 as u128, 1u16);
        // D s_9_34: cast reint s_9_32 -> u128
        let s_9_34: u128 = (s_9_32.value() as u128);
        // D s_9_35: size-of s_9_32
        let s_9_35: u16 = s_9_32.length();
        // D s_9_36: cast reint s_9_33 -> u128
        let s_9_36: u128 = (s_9_33.value() as u128);
        // D s_9_37: size-of s_9_33
        let s_9_37: u16 = s_9_33.length();
        // D s_9_38: lsl s_9_34 s_9_37
        let s_9_38: u128 = s_9_34 << s_9_37;
        // D s_9_39: or s_9_38 s_9_36
        let s_9_39: u128 = ((s_9_38) | (s_9_36));
        // D s_9_40: add s_9_35 s_9_37
        let s_9_40: u16 = (s_9_35 + s_9_37);
        // D s_9_41: create-bits s_9_39 s_9_40
        let s_9_41: Bits = Bits::new(s_9_39, s_9_40);
        // D s_9_42: cast reint s_9_41 -> u8
        let s_9_42: u8 = (s_9_41.value() as u8);
        // D s_9_43: cast zx s_9_27 -> bv
        let s_9_43: Bits = Bits::new(s_9_27 as u128, 1u16);
        // D s_9_44: cast zx s_9_42 -> bv
        let s_9_44: Bits = Bits::new(s_9_42 as u128, 2u16);
        // D s_9_45: cast reint s_9_43 -> u128
        let s_9_45: u128 = (s_9_43.value() as u128);
        // D s_9_46: size-of s_9_43
        let s_9_46: u16 = s_9_43.length();
        // D s_9_47: cast reint s_9_44 -> u128
        let s_9_47: u128 = (s_9_44.value() as u128);
        // D s_9_48: size-of s_9_44
        let s_9_48: u16 = s_9_44.length();
        // D s_9_49: lsl s_9_45 s_9_48
        let s_9_49: u128 = s_9_45 << s_9_48;
        // D s_9_50: or s_9_49 s_9_47
        let s_9_50: u128 = ((s_9_49) | (s_9_47));
        // D s_9_51: add s_9_46 s_9_48
        let s_9_51: u16 = (s_9_46 + s_9_48);
        // D s_9_52: create-bits s_9_50 s_9_51
        let s_9_52: Bits = Bits::new(s_9_50, s_9_51);
        // D s_9_53: cast reint s_9_52 -> u8
        let s_9_53: u8 = (s_9_52.value() as u8);
        // D s_9_54: write-var mask <= s_9_53
        fn_state.mask = s_9_53;
        // C s_9_55: const #16975u : u32
        let s_9_55: u32 = 16975;
        // D s_9_56: read-reg s_9_55:u8
        let s_9_56: u8 = {
            let value = state.read_register::<u8>(s_9_55 as isize);
            tracer.read_register(s_9_55 as isize, value);
            value
        };
        // D s_9_57: cast zx s_9_56 -> bv
        let s_9_57: Bits = Bits::new(s_9_56 as u128, 2u16);
        // C s_9_58: const #448u : u32
        let s_9_58: u32 = 448;
        // D s_9_59: read-reg s_9_58:u8
        let s_9_59: u8 = {
            let value = state.read_register::<u8>(s_9_58 as isize);
            tracer.read_register(s_9_58 as isize, value);
            value
        };
        // D s_9_60: cast zx s_9_59 -> bv
        let s_9_60: Bits = Bits::new(s_9_59 as u128, 2u16);
        // D s_9_61: cmp-eq s_9_57 s_9_60
        let s_9_61: bool = ((s_9_57) == (s_9_60));
        // N s_9_62: branch s_9_61 b55 b10
        if s_9_61 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 2u16);
        // C s_10_3: const #440u : u32
        let s_10_3: u32 = 440;
        // D s_10_4: read-reg s_10_3:u8
        let s_10_4: u8 = {
            let value = state.read_register::<u8>(s_10_3 as isize);
            tracer.read_register(s_10_3 as isize, value);
            value
        };
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 2u16);
        // D s_10_6: cmp-eq s_10_2 s_10_5
        let s_10_6: bool = ((s_10_2) == (s_10_5));
        // D s_10_7: write-var gs#327506 <= s_10_6
        fn_state.gs_327506 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_11_0: read-var gs#327506:u8
        let s_11_0: bool = fn_state.gs_327506;
        // N s_11_1: branch s_11_0 b54 b12
        if s_11_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#327507 <= s_12_0
        fn_state.gs_327507 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_13_0: read-var gs#327507:u8
        let s_13_0: bool = fn_state.gs_327507;
        // N s_13_1: branch s_13_0 b50 b14
        if s_13_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b49 b16
        if s_15_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#327508 <= s_16_0
        fn_state.gs_327508 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_17_0: read-var gs#327508:u8
        let s_17_0: bool = fn_state.gs_327508;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_19_0: read-var mask:u8
        let s_19_0: u8 = fn_state.mask;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 3u16);
        // D s_19_2: not s_19_1
        let s_19_2: Bits = !s_19_1;
        // D s_19_3: cast reint s_19_2 -> u8
        let s_19_3: u8 = (s_19_2.value() as u8);
        // D s_19_4: read-var pending:u8
        let s_19_4: u8 = fn_state.pending;
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 3u16);
        // D s_19_6: cast zx s_19_3 -> bv
        let s_19_6: Bits = Bits::new(s_19_3 as u128, 3u16);
        // D s_19_7: and s_19_5 s_19_6
        let s_19_7: Bits = ((s_19_5) & (s_19_6));
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: u8 = (s_19_7.value() as u8);
        // C s_19_9: const #2s : i
        let s_19_9: i128 = 2;
        // D s_19_10: cast zx s_19_8 -> bv
        let s_19_10: Bits = Bits::new(s_19_8 as u128, 3u16);
        // C s_19_11: const #1u : u64
        let s_19_11: u64 = 1;
        // D s_19_12: bit-extract s_19_10 s_19_9 s_19_11
        let s_19_12: Bits = (Bits::new(
            ((s_19_10) >> (s_19_9)).value(),
            u16::try_from(s_19_11).unwrap(),
        ));
        // D s_19_13: cast reint s_19_12 -> u8
        let s_19_13: bool = ((s_19_12.value()) != 0);
        // C s_19_14: const #0s : i
        let s_19_14: i128 = 0;
        // C s_19_15: const #0u : u64
        let s_19_15: u64 = 0;
        // D s_19_16: cast zx s_19_13 -> u64
        let s_19_16: u64 = (s_19_13 as u64);
        // C s_19_17: const #1u : u64
        let s_19_17: u64 = 1;
        // D s_19_18: and s_19_16 s_19_17
        let s_19_18: u64 = ((s_19_16) & (s_19_17));
        // D s_19_19: cmp-eq s_19_18 s_19_17
        let s_19_19: bool = ((s_19_18) == (s_19_17));
        // D s_19_20: lsl s_19_16 s_19_14
        let s_19_20: u64 = s_19_16 << s_19_14;
        // D s_19_21: or s_19_15 s_19_20
        let s_19_21: u64 = ((s_19_15) | (s_19_20));
        // D s_19_22: cmpl s_19_20
        let s_19_22: u64 = !s_19_20;
        // D s_19_23: and s_19_15 s_19_22
        let s_19_23: u64 = ((s_19_15) & (s_19_22));
        // D s_19_24: select s_19_19 s_19_21 s_19_23
        let s_19_24: u64 = if s_19_19 { s_19_21 } else { s_19_23 };
        // D s_19_25: cast trunc s_19_24 -> u8
        let s_19_25: bool = ((s_19_24) != 0);
        // D s_19_26: cast zx s_19_25 -> bv
        let s_19_26: Bits = Bits::new(s_19_25 as u128, 1u16);
        // C s_19_27: const #1u : u8
        let s_19_27: bool = true;
        // C s_19_28: cast zx s_19_27 -> bv
        let s_19_28: Bits = Bits::new(s_19_27 as u128, 1u16);
        // D s_19_29: cmp-eq s_19_26 s_19_28
        let s_19_29: bool = ((s_19_26) == (s_19_28));
        // C s_19_30: const #1s : i
        let s_19_30: i128 = 1;
        // D s_19_31: cast zx s_19_8 -> bv
        let s_19_31: Bits = Bits::new(s_19_8 as u128, 3u16);
        // C s_19_32: const #1u : u64
        let s_19_32: u64 = 1;
        // D s_19_33: bit-extract s_19_31 s_19_30 s_19_32
        let s_19_33: Bits = (Bits::new(
            ((s_19_31) >> (s_19_30)).value(),
            u16::try_from(s_19_32).unwrap(),
        ));
        // D s_19_34: cast reint s_19_33 -> u8
        let s_19_34: bool = ((s_19_33.value()) != 0);
        // C s_19_35: const #0s : i
        let s_19_35: i128 = 0;
        // C s_19_36: const #0u : u64
        let s_19_36: u64 = 0;
        // D s_19_37: cast zx s_19_34 -> u64
        let s_19_37: u64 = (s_19_34 as u64);
        // C s_19_38: const #1u : u64
        let s_19_38: u64 = 1;
        // D s_19_39: and s_19_37 s_19_38
        let s_19_39: u64 = ((s_19_37) & (s_19_38));
        // D s_19_40: cmp-eq s_19_39 s_19_38
        let s_19_40: bool = ((s_19_39) == (s_19_38));
        // D s_19_41: lsl s_19_37 s_19_35
        let s_19_41: u64 = s_19_37 << s_19_35;
        // D s_19_42: or s_19_36 s_19_41
        let s_19_42: u64 = ((s_19_36) | (s_19_41));
        // D s_19_43: cmpl s_19_41
        let s_19_43: u64 = !s_19_41;
        // D s_19_44: and s_19_36 s_19_43
        let s_19_44: u64 = ((s_19_36) & (s_19_43));
        // D s_19_45: select s_19_40 s_19_42 s_19_44
        let s_19_45: u64 = if s_19_40 { s_19_42 } else { s_19_44 };
        // D s_19_46: cast trunc s_19_45 -> u8
        let s_19_46: bool = ((s_19_45) != 0);
        // D s_19_47: cast zx s_19_46 -> bv
        let s_19_47: Bits = Bits::new(s_19_46 as u128, 1u16);
        // C s_19_48: const #1u : u8
        let s_19_48: bool = true;
        // C s_19_49: cast zx s_19_48 -> bv
        let s_19_49: Bits = Bits::new(s_19_48 as u128, 1u16);
        // D s_19_50: cmp-eq s_19_47 s_19_49
        let s_19_50: bool = ((s_19_47) == (s_19_49));
        // C s_19_51: const #0s : i
        let s_19_51: i128 = 0;
        // D s_19_52: cast zx s_19_8 -> bv
        let s_19_52: Bits = Bits::new(s_19_8 as u128, 3u16);
        // C s_19_53: const #1u : u64
        let s_19_53: u64 = 1;
        // D s_19_54: bit-extract s_19_52 s_19_51 s_19_53
        let s_19_54: Bits = (Bits::new(
            ((s_19_52) >> (s_19_51)).value(),
            u16::try_from(s_19_53).unwrap(),
        ));
        // D s_19_55: cast reint s_19_54 -> u8
        let s_19_55: bool = ((s_19_54.value()) != 0);
        // C s_19_56: const #0s : i
        let s_19_56: i128 = 0;
        // C s_19_57: const #0u : u64
        let s_19_57: u64 = 0;
        // D s_19_58: cast zx s_19_55 -> u64
        let s_19_58: u64 = (s_19_55 as u64);
        // C s_19_59: const #1u : u64
        let s_19_59: u64 = 1;
        // D s_19_60: and s_19_58 s_19_59
        let s_19_60: u64 = ((s_19_58) & (s_19_59));
        // D s_19_61: cmp-eq s_19_60 s_19_59
        let s_19_61: bool = ((s_19_60) == (s_19_59));
        // D s_19_62: lsl s_19_58 s_19_56
        let s_19_62: u64 = s_19_58 << s_19_56;
        // D s_19_63: or s_19_57 s_19_62
        let s_19_63: u64 = ((s_19_57) | (s_19_62));
        // D s_19_64: cmpl s_19_62
        let s_19_64: u64 = !s_19_62;
        // D s_19_65: and s_19_57 s_19_64
        let s_19_65: u64 = ((s_19_57) & (s_19_64));
        // D s_19_66: select s_19_61 s_19_63 s_19_65
        let s_19_66: u64 = if s_19_61 { s_19_63 } else { s_19_65 };
        // D s_19_67: cast trunc s_19_66 -> u8
        let s_19_67: bool = ((s_19_66) != 0);
        // D s_19_68: cast zx s_19_67 -> bv
        let s_19_68: Bits = Bits::new(s_19_67 as u128, 1u16);
        // C s_19_69: const #1u : u8
        let s_19_69: bool = true;
        // C s_19_70: cast zx s_19_69 -> bv
        let s_19_70: Bits = Bits::new(s_19_69 as u128, 1u16);
        // D s_19_71: cmp-eq s_19_68 s_19_70
        let s_19_71: bool = ((s_19_68) == (s_19_70));
        // D s_19_72: create-product struct = ["s_19_29", "s_19_50", "s_19_71"]
        let s_19_72: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_19_29,
            _1: s_19_50,
            _2: s_19_71,
        };
        // D s_19_73: write-var return_value <= s_19_72
        fn_state.return_value = s_19_72;
        // N s_19_74: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_20_0: read-var return_value:struct
        let s_20_0: ProductTyped8f896a024a4e2cb = fn_state.return_value;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_21_0: const #16975u : u32
        let s_21_0: u32 = 16975;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 2u16);
        // C s_21_3: const #424u : u32
        let s_21_3: u32 = 424;
        // D s_21_4: read-reg s_21_3:u8
        let s_21_4: u8 = {
            let value = state.read_register::<u8>(s_21_3 as isize);
            tracer.read_register(s_21_3 as isize, value);
            value
        };
        // D s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 2u16);
        // D s_21_6: cmp-ne s_21_2 s_21_5
        let s_21_6: bool = ((s_21_2) != (s_21_5));
        // N s_21_7: branch s_21_6 b24 b22
        if s_21_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_23_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_24_0: const #20920u : u32
        let s_24_0: u32 = 20920;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_SCR_Type_FIQ(s_24_1)
        let s_24_2: bool = u_get_SCR_Type_FIQ(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // D s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // N s_24_7: branch s_24_6 b45 b25
        if s_24_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#327512 <= s_25_0
        fn_state.gs_327512 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_26_0: read-var gs#327512:u8
        let s_26_0: bool = fn_state.gs_327512;
        // N s_26_1: branch s_26_0 b44 b27
        if s_26_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_28_0: const #20920u : u32
        let s_28_0: u32 = 20920;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_SCR_Type_IRQ(s_28_1)
        let s_28_2: bool = u_get_SCR_Type_IRQ(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // N s_28_7: branch s_28_6 b43 b29
        if s_28_6 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#327513 <= s_29_0
        fn_state.gs_327513 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_30_0: read-var gs#327513:u8
        let s_30_0: bool = fn_state.gs_327513;
        // N s_30_1: branch s_30_0 b42 b31
        if s_30_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EffectiveEA(s_32_0)
        let s_32_1: bool = EffectiveEA(state, tracer, s_32_0);
        // S s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 1u16);
        // C s_32_3: const #1u : u8
        let s_32_3: bool = true;
        // C s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 1u16);
        // S s_32_5: cmp-eq s_32_2 s_32_4
        let s_32_5: bool = ((s_32_2) == (s_32_4));
        // N s_32_6: branch s_32_5 b38 b33
        if s_32_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#327515 <= s_33_0
        fn_state.gs_327515 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_34_0: read-var gs#327515:u8
        let s_34_0: bool = fn_state.gs_327515;
        // N s_34_1: branch s_34_0 b37 b35
        if s_34_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_36_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // S s_37_1: call Bit(s_37_0)
        let s_37_1: bool = Bit(state, tracer, s_37_0);
        // C s_37_2: const #2s : i
        let s_37_2: i128 = 2;
        // D s_37_3: read-var mask:u8
        let s_37_3: u8 = fn_state.mask;
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 3u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-insert s_37_4 s_37_4 s_37_2 s_37_5
        let s_37_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_37_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_37_4.length(),
            );
            (s_37_4 & mask) | (s_37_4 << s_37_2)
        };
        // D s_37_7: cast reint s_37_6 -> u8
        let s_37_7: u8 = (s_37_6.value() as u8);
        // D s_37_8: write-var mask <= s_37_7
        fn_state.mask = s_37_7;
        // N s_37_9: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_38_0: const #20920u : u32
        let s_38_0: u32 = 20920;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_SCR_Type_AW(s_38_1)
        let s_38_2: bool = u_get_SCR_Type_AW(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #0u : u8
        let s_38_4: bool = false;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // N s_38_7: branch s_38_6 b41 b39
        if s_38_6 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HCR_read(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_39_0);
        // S s_39_2: call _get_HCR_Type_AMO(s_39_1)
        let s_39_2: bool = u_get_HCR_Type_AMO(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#327514 <= s_39_6
        fn_state.gs_327514 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_40_0: read-var gs#327514:u8
        let s_40_0: bool = fn_state.gs_327514;
        // D s_40_1: write-var gs#327515 <= s_40_0
        fn_state.gs_327515 = s_40_0;
        // N s_40_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#327514 <= s_41_0
        fn_state.gs_327514 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // S s_42_1: call Bit(s_42_0)
        let s_42_1: bool = Bit(state, tracer, s_42_0);
        // C s_42_2: const #1s : i
        let s_42_2: i128 = 1;
        // D s_42_3: read-var mask:u8
        let s_42_3: u8 = fn_state.mask;
        // D s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 3u16);
        // C s_42_5: const #1u : u64
        let s_42_5: u64 = 1;
        // D s_42_6: bit-insert s_42_4 s_42_4 s_42_2 s_42_5
        let s_42_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_42_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_42_4.length(),
            );
            (s_42_4 & mask) | (s_42_4 << s_42_2)
        };
        // D s_42_7: cast reint s_42_6 -> u8
        let s_42_7: u8 = (s_42_6.value() as u8);
        // D s_42_8: write-var mask <= s_42_7
        fn_state.mask = s_42_7;
        // N s_42_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call HCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_HCR_Type_IMO(s_43_1)
        let s_43_2: bool = u_get_HCR_Type_IMO(state, tracer, s_43_1);
        // S s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // S s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#327513 <= s_43_6
        fn_state.gs_327513 = s_43_6;
        // N s_43_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // S s_44_1: call Bit(s_44_0)
        let s_44_1: bool = Bit(state, tracer, s_44_0);
        // C s_44_2: const #0s : i
        let s_44_2: i128 = 0;
        // D s_44_3: read-var mask:u8
        let s_44_3: u8 = fn_state.mask;
        // D s_44_4: cast zx s_44_3 -> bv
        let s_44_4: Bits = Bits::new(s_44_3 as u128, 3u16);
        // C s_44_5: const #1u : u64
        let s_44_5: u64 = 1;
        // D s_44_6: bit-insert s_44_4 s_44_4 s_44_2 s_44_5
        let s_44_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_44_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_44_4.length(),
            );
            (s_44_4 & mask) | (s_44_4 << s_44_2)
        };
        // D s_44_7: cast reint s_44_6 -> u8
        let s_44_7: u8 = (s_44_6.value() as u8);
        // D s_44_8: write-var mask <= s_44_7
        fn_state.mask = s_44_7;
        // N s_44_9: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_45_0: const #20920u : u32
        let s_45_0: u32 = 20920;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_SCR_Type_FW(s_45_1)
        let s_45_2: bool = u_get_SCR_Type_FW(state, tracer, s_45_1);
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #0u : u8
        let s_45_4: bool = false;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // D s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // N s_45_7: branch s_45_6 b48 b46
        if s_45_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call HCR_read(s_46_0)
        let s_46_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_46_0);
        // S s_46_2: call _get_HCR_Type_FMO(s_46_1)
        let s_46_2: bool = u_get_HCR_Type_FMO(state, tracer, s_46_1);
        // S s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #1u : u8
        let s_46_4: bool = true;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // S s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var gs#327511 <= s_46_6
        fn_state.gs_327511 = s_46_6;
        // N s_46_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_47_0: read-var gs#327511:u8
        let s_47_0: bool = fn_state.gs_327511;
        // D s_47_1: write-var gs#327512 <= s_47_0
        fn_state.gs_327512 = s_47_0;
        // N s_47_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#327511 <= s_48_0
        fn_state.gs_327511 = s_48_0;
        // N s_48_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call CurrentSecurityState(s_49_0)
        let s_49_1: u32 = CurrentSecurityState(state, tracer, s_49_0);
        // C s_49_2: const #3u : u32
        let s_49_2: u32 = 3;
        // S s_49_3: cmp-eq s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) == (s_49_2));
        // D s_49_4: write-var gs#327508 <= s_49_3
        fn_state.gs_327508 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call HCR_read(s_50_0)
        let s_50_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_50_0);
        // S s_50_2: call _get_HCR_Type_TGE(s_50_1)
        let s_50_2: bool = u_get_HCR_Type_TGE(state, tracer, s_50_1);
        // S s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // C s_50_4: const #1u : u8
        let s_50_4: bool = true;
        // C s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 1u16);
        // S s_50_6: cmp-eq s_50_3 s_50_5
        let s_50_6: bool = ((s_50_3) == (s_50_5));
        // N s_50_7: branch s_50_6 b53 b51
        if s_50_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call HCR_read(s_51_0)
        let s_51_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_51_0);
        // S s_51_2: call _get_HCR_Type_AMO(s_51_1)
        let s_51_2: bool = u_get_HCR_Type_AMO(state, tracer, s_51_1);
        // C s_51_3: const #() : ()
        let s_51_3: () = ();
        // S s_51_4: call HCR_read(s_51_3)
        let s_51_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_51_3);
        // S s_51_5: call _get_HCR_Type_IMO(s_51_4)
        let s_51_5: bool = u_get_HCR_Type_IMO(state, tracer, s_51_4);
        // C s_51_6: const #() : ()
        let s_51_6: () = ();
        // S s_51_7: call HCR_read(s_51_6)
        let s_51_7: ProductType700c18a878c5601b = HCR_read(state, tracer, s_51_6);
        // S s_51_8: call _get_HCR_Type_FMO(s_51_7)
        let s_51_8: bool = u_get_HCR_Type_FMO(state, tracer, s_51_7);
        // S s_51_9: cast zx s_51_5 -> bv
        let s_51_9: Bits = Bits::new(s_51_5 as u128, 1u16);
        // S s_51_10: cast zx s_51_8 -> bv
        let s_51_10: Bits = Bits::new(s_51_8 as u128, 1u16);
        // S s_51_11: cast reint s_51_9 -> u128
        let s_51_11: u128 = (s_51_9.value() as u128);
        // D s_51_12: size-of s_51_9
        let s_51_12: u16 = s_51_9.length();
        // S s_51_13: cast reint s_51_10 -> u128
        let s_51_13: u128 = (s_51_10.value() as u128);
        // D s_51_14: size-of s_51_10
        let s_51_14: u16 = s_51_10.length();
        // D s_51_15: lsl s_51_11 s_51_14
        let s_51_15: u128 = s_51_11 << s_51_14;
        // D s_51_16: or s_51_15 s_51_13
        let s_51_16: u128 = ((s_51_15) | (s_51_13));
        // D s_51_17: add s_51_12 s_51_14
        let s_51_17: u16 = (s_51_12 + s_51_14);
        // D s_51_18: create-bits s_51_16 s_51_17
        let s_51_18: Bits = Bits::new(s_51_16, s_51_17);
        // D s_51_19: cast reint s_51_18 -> u8
        let s_51_19: u8 = (s_51_18.value() as u8);
        // S s_51_20: cast zx s_51_2 -> bv
        let s_51_20: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_21: cast zx s_51_19 -> bv
        let s_51_21: Bits = Bits::new(s_51_19 as u128, 2u16);
        // S s_51_22: cast reint s_51_20 -> u128
        let s_51_22: u128 = (s_51_20.value() as u128);
        // D s_51_23: size-of s_51_20
        let s_51_23: u16 = s_51_20.length();
        // D s_51_24: cast reint s_51_21 -> u128
        let s_51_24: u128 = (s_51_21.value() as u128);
        // D s_51_25: size-of s_51_21
        let s_51_25: u16 = s_51_21.length();
        // D s_51_26: lsl s_51_22 s_51_25
        let s_51_26: u128 = s_51_22 << s_51_25;
        // D s_51_27: or s_51_26 s_51_24
        let s_51_27: u128 = ((s_51_26) | (s_51_24));
        // D s_51_28: add s_51_23 s_51_25
        let s_51_28: u16 = (s_51_23 + s_51_25);
        // D s_51_29: create-bits s_51_27 s_51_28
        let s_51_29: Bits = Bits::new(s_51_27, s_51_28);
        // D s_51_30: cast reint s_51_29 -> u8
        let s_51_30: u8 = (s_51_29.value() as u8);
        // D s_51_31: write-var mask_overrideshadow#7999 <= s_51_30
        fn_state.mask_overrideshadow_7999 = s_51_30;
        // N s_51_32: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_52_0: read-var mask_overrideshadow#7999:u8
        let s_52_0: u8 = fn_state.mask_overrideshadow_7999;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 3u16);
        // D s_52_2: not s_52_1
        let s_52_2: Bits = !s_52_1;
        // D s_52_3: cast reint s_52_2 -> u8
        let s_52_3: u8 = (s_52_2.value() as u8);
        // D s_52_4: read-var mask:u8
        let s_52_4: u8 = fn_state.mask;
        // D s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 3u16);
        // D s_52_6: cast zx s_52_3 -> bv
        let s_52_6: Bits = Bits::new(s_52_3 as u128, 3u16);
        // D s_52_7: and s_52_5 s_52_6
        let s_52_7: Bits = ((s_52_5) & (s_52_6));
        // D s_52_8: cast reint s_52_7 -> u8
        let s_52_8: u8 = (s_52_7.value() as u8);
        // D s_52_9: write-var mask <= s_52_8
        fn_state.mask = s_52_8;
        // N s_52_10: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_53_0: const #7u : u8
        let s_53_0: u8 = 7;
        // D s_53_1: write-var mask_overrideshadow#7999 <= s_53_0
        fn_state.mask_overrideshadow_7999 = s_53_0;
        // N s_53_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // D s_54_2: write-var gs#327507 <= s_54_1
        fn_state.gs_327507 = s_54_1;
        // N s_54_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#327506 <= s_55_0
        fn_state.gs_327506 = s_55_0;
        // N s_55_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var fiq_pending <= s_56_0
        fn_state.fiq_pending = s_56_0;
        // N s_56_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var irq_pending <= s_57_0
        fn_state.irq_pending = s_57_0;
        // N s_57_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var se_pending <= s_58_0
        fn_state.se_pending = s_58_0;
        // N s_58_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_59_0: const #16968u : u32
        let s_59_0: u32 = 16968;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: bool = {
            let value = state.read_register::<bool>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #16979u : u32
        let s_59_2: u32 = 16979;
        // D s_59_3: read-reg s_59_2:u8
        let s_59_3: bool = {
            let value = state.read_register::<bool>(s_59_2 as isize);
            tracer.read_register(s_59_2 as isize, value);
            value
        };
        // C s_59_4: const #16977u : u32
        let s_59_4: u32 = 16977;
        // D s_59_5: read-reg s_59_4:u8
        let s_59_5: bool = {
            let value = state.read_register::<bool>(s_59_4 as isize);
            tracer.read_register(s_59_4 as isize, value);
            value
        };
        // D s_59_6: cast zx s_59_3 -> bv
        let s_59_6: Bits = Bits::new(s_59_3 as u128, 1u16);
        // D s_59_7: cast zx s_59_5 -> bv
        let s_59_7: Bits = Bits::new(s_59_5 as u128, 1u16);
        // D s_59_8: cast reint s_59_6 -> u128
        let s_59_8: u128 = (s_59_6.value() as u128);
        // D s_59_9: size-of s_59_6
        let s_59_9: u16 = s_59_6.length();
        // D s_59_10: cast reint s_59_7 -> u128
        let s_59_10: u128 = (s_59_7.value() as u128);
        // D s_59_11: size-of s_59_7
        let s_59_11: u16 = s_59_7.length();
        // D s_59_12: lsl s_59_8 s_59_11
        let s_59_12: u128 = s_59_8 << s_59_11;
        // D s_59_13: or s_59_12 s_59_10
        let s_59_13: u128 = ((s_59_12) | (s_59_10));
        // D s_59_14: add s_59_9 s_59_11
        let s_59_14: u16 = (s_59_9 + s_59_11);
        // D s_59_15: create-bits s_59_13 s_59_14
        let s_59_15: Bits = Bits::new(s_59_13, s_59_14);
        // D s_59_16: cast reint s_59_15 -> u8
        let s_59_16: u8 = (s_59_15.value() as u8);
        // D s_59_17: cast zx s_59_1 -> bv
        let s_59_17: Bits = Bits::new(s_59_1 as u128, 1u16);
        // D s_59_18: cast zx s_59_16 -> bv
        let s_59_18: Bits = Bits::new(s_59_16 as u128, 2u16);
        // D s_59_19: cast reint s_59_17 -> u128
        let s_59_19: u128 = (s_59_17.value() as u128);
        // D s_59_20: size-of s_59_17
        let s_59_20: u16 = s_59_17.length();
        // D s_59_21: cast reint s_59_18 -> u128
        let s_59_21: u128 = (s_59_18.value() as u128);
        // D s_59_22: size-of s_59_18
        let s_59_22: u16 = s_59_18.length();
        // D s_59_23: lsl s_59_19 s_59_22
        let s_59_23: u128 = s_59_19 << s_59_22;
        // D s_59_24: or s_59_23 s_59_21
        let s_59_24: u128 = ((s_59_23) | (s_59_21));
        // D s_59_25: add s_59_20 s_59_22
        let s_59_25: u16 = (s_59_20 + s_59_22);
        // D s_59_26: create-bits s_59_24 s_59_25
        let s_59_26: Bits = Bits::new(s_59_24, s_59_25);
        // D s_59_27: cast reint s_59_26 -> u8
        let s_59_27: u8 = (s_59_26.value() as u8);
        // D s_59_28: call AArch64_PendingUnmaskedPhysicalInterrupts(s_59_27)
        let s_59_28: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_59_27,
        );
        // D s_59_29: write-var return_value <= s_59_28
        fn_state.return_value = s_59_28;
        // N s_59_30: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_60_0: const #424u : u32
        let s_60_0: u32 = 424;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call ELUsingAArch32(s_60_1)
        let s_60_2: bool = ELUsingAArch32(state, tracer, s_60_1);
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // D s_60_4: write-var gs#327498 <= s_60_3
        fn_state.gs_327498 = s_60_3;
        // N s_60_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
