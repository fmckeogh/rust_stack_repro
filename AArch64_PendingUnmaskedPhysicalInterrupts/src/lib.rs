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
use EL2Enabled::*;
use FIQPending::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCTLRType_SPINTMASK::*;
use IRQPending::*;
use u_get_SCR_EL3_Type_IRQ::*;
use HaveFeatNMI::*;
use u_get_SCTLRType_NMI::*;
use u_get_SCR_EL3_Type_FIQ::*;
use u_get_HCR_EL2_Type_FMO::*;
use SCTLR_read__1::*;
use ELUsingAArch32::*;
use u_get_SCR_EL3_Type_EA::*;
use IsPhysicalSErrorPending::*;
use Bit::*;
use u_get_HCR_EL2_Type_IMO::*;
use u_get_HCR_EL2_Type_TGE::*;
use HaveVirtHostExt::*;
use u_get_HCR_EL2_Type_AMO::*;
use common::*;
pub fn AArch64_PendingUnmaskedPhysicalInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mask_in: u8,
) -> ProductTyped8f896a024a4e2cb {
    #[derive(Default)]
    struct FunctionState {
        se_pending: bool,
        fiq_nmi: bool,
        allintmask: bool,
        gs_327439: bool,
        pending: u8,
        gs_327460: bool,
        mask: u8,
        ga_367537: ProductType8b847afc727d5818,
        gs_327432: bool,
        gs_327461: bool,
        gs_327458: bool,
        mask_override: u8,
        gs_327450: bool,
        irq_pending: bool,
        gs_327430: bool,
        gs_327449: bool,
        ga_367536: ProductType8b847afc727d5818,
        gs_327457: bool,
        fiq: bool,
        fiq_pending: bool,
        irq_nmi: bool,
        mask_in: u8,
    }
    let fn_state = FunctionState {
        mask_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_0_0: read-var mask_in:u8
        let s_0_0: u8 = fn_state.mask_in;
        // D s_0_1: write-var mask <= s_0_0
        fn_state.mask = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call IsPhysicalSErrorPending(s_0_2)
        let s_0_3: bool = IsPhysicalSErrorPending(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b93 b1
        if s_0_3 {
            return block_93(state, tracer, fn_state);
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
        // D s_1_1: write-var se_pending <= s_1_0
        fn_state.se_pending = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call IRQPending(s_2_0)
        let s_2_1: ProductType8b847afc727d5818 = IRQPending(state, tracer, s_2_0);
        // D s_2_2: write-var ga#367536 <= s_2_1
        fn_state.ga_367536 = s_2_1;
        // D s_2_3: read-var ga#367536.0:struct
        let s_2_3: bool = fn_state.ga_367536._0;
        // D s_2_4: read-var ga#367536.1:struct
        let s_2_4: bool = fn_state.ga_367536._1;
        // D s_2_5: write-var irq_nmi <= s_2_4
        fn_state.irq_nmi = s_2_4;
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call FIQPending(s_2_6)
        let s_2_7: ProductType8b847afc727d5818 = FIQPending(state, tracer, s_2_6);
        // D s_2_8: write-var ga#367537 <= s_2_7
        fn_state.ga_367537 = s_2_7;
        // D s_2_9: read-var ga#367537.0:struct
        let s_2_9: bool = fn_state.ga_367537._0;
        // D s_2_10: read-var ga#367537.1:struct
        let s_2_10: bool = fn_state.ga_367537._1;
        // D s_2_11: write-var fiq <= s_2_9
        fn_state.fiq = s_2_9;
        // D s_2_12: write-var fiq_nmi <= s_2_10
        fn_state.fiq_nmi = s_2_10;
        // N s_2_13: branch s_2_3 b92 b3
        if s_2_3 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var irq_pending <= s_3_0
        fn_state.irq_pending = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_4_0: read-var fiq:u8
        let s_4_0: bool = fn_state.fiq;
        // N s_4_1: branch s_4_0 b91 b5
        if s_4_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var fiq_pending <= s_5_0
        fn_state.fiq_pending = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveFeatNMI(s_6_0)
        let s_6_1: bool = HaveFeatNMI(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b90 b7
        if s_6_1 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#327430 <= s_7_0
        fn_state.gs_327430 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_8_0: read-var gs#327430:u8
        let s_8_0: bool = fn_state.gs_327430;
        // N s_8_1: branch s_8_0 b77 b9
        if s_8_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var allintmask <= s_9_0
        fn_state.allintmask = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_10_0: read-var se_pending:u8
        let s_10_0: bool = fn_state.se_pending;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // D s_10_2: read-var irq_pending:u8
        let s_10_2: bool = fn_state.irq_pending;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
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
        let s_10_13: Bits = Bits::new(s_10_12 as u128, 2u16);
        // D s_10_14: read-var fiq_pending:u8
        let s_10_14: bool = fn_state.fiq_pending;
        // D s_10_15: cast zx s_10_14 -> bv
        let s_10_15: Bits = Bits::new(s_10_14 as u128, 1u16);
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
        // D s_10_25: write-var pending <= s_10_24
        fn_state.pending = s_10_24;
        // C s_10_26: const #() : ()
        let s_10_26: () = ();
        // S s_10_27: call EL2Enabled(s_10_26)
        let s_10_27: bool = EL2Enabled(state, tracer, s_10_26);
        // N s_10_28: branch s_10_27 b52 b11
        if s_10_27 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #2u : u8
        let s_12_2: u8 = 2;
        // D s_12_3: cmp-lt s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) < (s_12_2));
        // N s_12_4: branch s_12_3 b49 b13
        if s_12_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_14_0: const #16975u : u32
        let s_14_0: u32 = 16975;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
        // C s_14_3: const #448u : u32
        let s_14_3: u32 = 448;
        // D s_14_4: read-reg s_14_3:u8
        let s_14_4: u8 = {
            let value = state.read_register::<u8>(s_14_3 as isize);
            tracer.read_register(s_14_3 as isize, value);
            value
        };
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 2u16);
        // D s_14_6: cmp-ne s_14_2 s_14_5
        let s_14_6: bool = ((s_14_2) != (s_14_5));
        // N s_14_7: branch s_14_6 b48 b15
        if s_14_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#327439 <= s_15_0
        fn_state.gs_327439 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_16_0: read-var gs#327439:u8
        let s_16_0: bool = fn_state.gs_327439;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_18_0: read-var mask:u8
        let s_18_0: u8 = fn_state.mask;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 3u16);
        // D s_18_2: not s_18_1
        let s_18_2: Bits = !s_18_1;
        // D s_18_3: cast reint s_18_2 -> u8
        let s_18_3: u8 = (s_18_2.value() as u8);
        // D s_18_4: read-var pending:u8
        let s_18_4: u8 = fn_state.pending;
        // D s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 3u16);
        // D s_18_6: cast zx s_18_3 -> bv
        let s_18_6: Bits = Bits::new(s_18_3 as u128, 3u16);
        // D s_18_7: and s_18_5 s_18_6
        let s_18_7: Bits = ((s_18_5) & (s_18_6));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // C s_18_9: const #2s : i
        let s_18_9: i128 = 2;
        // D s_18_10: cast zx s_18_8 -> bv
        let s_18_10: Bits = Bits::new(s_18_8 as u128, 3u16);
        // C s_18_11: const #1u : u64
        let s_18_11: u64 = 1;
        // D s_18_12: bit-extract s_18_10 s_18_9 s_18_11
        let s_18_12: Bits = (Bits::new(
            ((s_18_10) >> (s_18_9)).value(),
            u16::try_from(s_18_11).unwrap(),
        ));
        // D s_18_13: cast reint s_18_12 -> u8
        let s_18_13: bool = ((s_18_12.value()) != 0);
        // C s_18_14: const #0s : i
        let s_18_14: i128 = 0;
        // C s_18_15: const #0u : u64
        let s_18_15: u64 = 0;
        // D s_18_16: cast zx s_18_13 -> u64
        let s_18_16: u64 = (s_18_13 as u64);
        // C s_18_17: const #1u : u64
        let s_18_17: u64 = 1;
        // D s_18_18: and s_18_16 s_18_17
        let s_18_18: u64 = ((s_18_16) & (s_18_17));
        // D s_18_19: cmp-eq s_18_18 s_18_17
        let s_18_19: bool = ((s_18_18) == (s_18_17));
        // D s_18_20: lsl s_18_16 s_18_14
        let s_18_20: u64 = s_18_16 << s_18_14;
        // D s_18_21: or s_18_15 s_18_20
        let s_18_21: u64 = ((s_18_15) | (s_18_20));
        // D s_18_22: cmpl s_18_20
        let s_18_22: u64 = !s_18_20;
        // D s_18_23: and s_18_15 s_18_22
        let s_18_23: u64 = ((s_18_15) & (s_18_22));
        // D s_18_24: select s_18_19 s_18_21 s_18_23
        let s_18_24: u64 = if s_18_19 { s_18_21 } else { s_18_23 };
        // D s_18_25: cast trunc s_18_24 -> u8
        let s_18_25: bool = ((s_18_24) != 0);
        // D s_18_26: cast zx s_18_25 -> bv
        let s_18_26: Bits = Bits::new(s_18_25 as u128, 1u16);
        // C s_18_27: const #1u : u8
        let s_18_27: bool = true;
        // C s_18_28: cast zx s_18_27 -> bv
        let s_18_28: Bits = Bits::new(s_18_27 as u128, 1u16);
        // D s_18_29: cmp-eq s_18_26 s_18_28
        let s_18_29: bool = ((s_18_26) == (s_18_28));
        // C s_18_30: const #1s : i
        let s_18_30: i128 = 1;
        // D s_18_31: cast zx s_18_8 -> bv
        let s_18_31: Bits = Bits::new(s_18_8 as u128, 3u16);
        // C s_18_32: const #1u : u64
        let s_18_32: u64 = 1;
        // D s_18_33: bit-extract s_18_31 s_18_30 s_18_32
        let s_18_33: Bits = (Bits::new(
            ((s_18_31) >> (s_18_30)).value(),
            u16::try_from(s_18_32).unwrap(),
        ));
        // D s_18_34: cast reint s_18_33 -> u8
        let s_18_34: bool = ((s_18_33.value()) != 0);
        // C s_18_35: const #0s : i
        let s_18_35: i128 = 0;
        // C s_18_36: const #0u : u64
        let s_18_36: u64 = 0;
        // D s_18_37: cast zx s_18_34 -> u64
        let s_18_37: u64 = (s_18_34 as u64);
        // C s_18_38: const #1u : u64
        let s_18_38: u64 = 1;
        // D s_18_39: and s_18_37 s_18_38
        let s_18_39: u64 = ((s_18_37) & (s_18_38));
        // D s_18_40: cmp-eq s_18_39 s_18_38
        let s_18_40: bool = ((s_18_39) == (s_18_38));
        // D s_18_41: lsl s_18_37 s_18_35
        let s_18_41: u64 = s_18_37 << s_18_35;
        // D s_18_42: or s_18_36 s_18_41
        let s_18_42: u64 = ((s_18_36) | (s_18_41));
        // D s_18_43: cmpl s_18_41
        let s_18_43: u64 = !s_18_41;
        // D s_18_44: and s_18_36 s_18_43
        let s_18_44: u64 = ((s_18_36) & (s_18_43));
        // D s_18_45: select s_18_40 s_18_42 s_18_44
        let s_18_45: u64 = if s_18_40 { s_18_42 } else { s_18_44 };
        // D s_18_46: cast trunc s_18_45 -> u8
        let s_18_46: bool = ((s_18_45) != 0);
        // D s_18_47: cast zx s_18_46 -> bv
        let s_18_47: Bits = Bits::new(s_18_46 as u128, 1u16);
        // C s_18_48: const #1u : u8
        let s_18_48: bool = true;
        // C s_18_49: cast zx s_18_48 -> bv
        let s_18_49: Bits = Bits::new(s_18_48 as u128, 1u16);
        // D s_18_50: cmp-eq s_18_47 s_18_49
        let s_18_50: bool = ((s_18_47) == (s_18_49));
        // C s_18_51: const #0s : i
        let s_18_51: i128 = 0;
        // D s_18_52: cast zx s_18_8 -> bv
        let s_18_52: Bits = Bits::new(s_18_8 as u128, 3u16);
        // C s_18_53: const #1u : u64
        let s_18_53: u64 = 1;
        // D s_18_54: bit-extract s_18_52 s_18_51 s_18_53
        let s_18_54: Bits = (Bits::new(
            ((s_18_52) >> (s_18_51)).value(),
            u16::try_from(s_18_53).unwrap(),
        ));
        // D s_18_55: cast reint s_18_54 -> u8
        let s_18_55: bool = ((s_18_54.value()) != 0);
        // C s_18_56: const #0s : i
        let s_18_56: i128 = 0;
        // C s_18_57: const #0u : u64
        let s_18_57: u64 = 0;
        // D s_18_58: cast zx s_18_55 -> u64
        let s_18_58: u64 = (s_18_55 as u64);
        // C s_18_59: const #1u : u64
        let s_18_59: u64 = 1;
        // D s_18_60: and s_18_58 s_18_59
        let s_18_60: u64 = ((s_18_58) & (s_18_59));
        // D s_18_61: cmp-eq s_18_60 s_18_59
        let s_18_61: bool = ((s_18_60) == (s_18_59));
        // D s_18_62: lsl s_18_58 s_18_56
        let s_18_62: u64 = s_18_58 << s_18_56;
        // D s_18_63: or s_18_57 s_18_62
        let s_18_63: u64 = ((s_18_57) | (s_18_62));
        // D s_18_64: cmpl s_18_62
        let s_18_64: u64 = !s_18_62;
        // D s_18_65: and s_18_57 s_18_64
        let s_18_65: u64 = ((s_18_57) & (s_18_64));
        // D s_18_66: select s_18_61 s_18_63 s_18_65
        let s_18_66: u64 = if s_18_61 { s_18_63 } else { s_18_65 };
        // D s_18_67: cast trunc s_18_66 -> u8
        let s_18_67: bool = ((s_18_66) != 0);
        // D s_18_68: cast zx s_18_67 -> bv
        let s_18_68: Bits = Bits::new(s_18_67 as u128, 1u16);
        // C s_18_69: const #1u : u8
        let s_18_69: bool = true;
        // C s_18_70: cast zx s_18_69 -> bv
        let s_18_70: Bits = Bits::new(s_18_69 as u128, 1u16);
        // D s_18_71: cmp-eq s_18_68 s_18_70
        let s_18_71: bool = ((s_18_68) == (s_18_70));
        // D s_18_72: create-product struct = ["s_18_29", "s_18_50", "s_18_71"]
        let s_18_72: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_18_29,
            _1: s_18_50,
            _2: s_18_71,
        };
        // N s_18_73: return s_18_72
        return s_18_72;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_19_0: const #16975u : u32
        let s_19_0: u32 = 16975;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 2u16);
        // C s_19_3: const #440u : u32
        let s_19_3: u32 = 440;
        // D s_19_4: read-reg s_19_3:u8
        let s_19_4: u8 = {
            let value = state.read_register::<u8>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 2u16);
        // D s_19_6: cmp-eq s_19_2 s_19_5
        let s_19_6: bool = ((s_19_2) == (s_19_5));
        // N s_19_7: branch s_19_6 b35 b20
        if s_19_6 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
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
        // C s_20_3: const #432u : u32
        let s_20_3: u32 = 432;
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
        // N s_20_7: branch s_20_6 b28 b21
        if s_20_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_21_0: const #90704u : u32
        let s_21_0: u32 = 90704;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_SCR_EL3_Type_FIQ(s_21_1)
        let s_21_2: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // N s_21_7: branch s_21_6 b27 b22
        if s_21_6 {
            return block_27(state, tracer, fn_state);
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
        // C s_23_0: const #90704u : u32
        let s_23_0: u32 = 90704;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SCR_EL3_Type_IRQ(s_23_1)
        let s_23_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // N s_23_7: branch s_23_6 b26 b24
        if s_23_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_25_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // S s_26_1: call Bit(s_26_0)
        let s_26_1: bool = Bit(state, tracer, s_26_0);
        // C s_26_2: const #1s : i
        let s_26_2: i128 = 1;
        // D s_26_3: read-var mask:u8
        let s_26_3: u8 = fn_state.mask;
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 3u16);
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-insert s_26_4 s_26_4 s_26_2 s_26_5
        let s_26_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_26_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_26_4.length(),
            );
            (s_26_4 & mask) | (s_26_4 << s_26_2)
        };
        // D s_26_7: cast reint s_26_6 -> u8
        let s_26_7: u8 = (s_26_6.value() as u8);
        // D s_26_8: write-var mask <= s_26_7
        fn_state.mask = s_26_7;
        // N s_26_9: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // S s_27_1: call Bit(s_27_0)
        let s_27_1: bool = Bit(state, tracer, s_27_0);
        // C s_27_2: const #0s : i
        let s_27_2: i128 = 0;
        // D s_27_3: read-var mask:u8
        let s_27_3: u8 = fn_state.mask;
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 3u16);
        // C s_27_5: const #1u : u64
        let s_27_5: u64 = 1;
        // D s_27_6: bit-insert s_27_4 s_27_4 s_27_2 s_27_5
        let s_27_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_4.length(),
            );
            (s_27_4 & mask) | (s_27_4 << s_27_2)
        };
        // D s_27_7: cast reint s_27_6 -> u8
        let s_27_7: u8 = (s_27_6.value() as u8);
        // D s_27_8: write-var mask <= s_27_7
        fn_state.mask = s_27_7;
        // N s_27_9: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_FMO(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // N s_28_7: branch s_28_6 b34 b29
        if s_28_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_30_0: const #102552u : u32
        let s_30_0: u32 = 102552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_HCR_EL2_Type_IMO(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #1u : u8
        let s_30_4: bool = true;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // N s_30_7: branch s_30_6 b33 b31
        if s_30_6 {
            return block_33(state, tracer, fn_state);
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
        // N s_32_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // S s_33_1: call Bit(s_33_0)
        let s_33_1: bool = Bit(state, tracer, s_33_0);
        // C s_33_2: const #1s : i
        let s_33_2: i128 = 1;
        // D s_33_3: read-var mask:u8
        let s_33_3: u8 = fn_state.mask;
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 3u16);
        // C s_33_5: const #1u : u64
        let s_33_5: u64 = 1;
        // D s_33_6: bit-insert s_33_4 s_33_4 s_33_2 s_33_5
        let s_33_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_33_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_33_4.length(),
            );
            (s_33_4 & mask) | (s_33_4 << s_33_2)
        };
        // D s_33_7: cast reint s_33_6 -> u8
        let s_33_7: u8 = (s_33_6.value() as u8);
        // D s_33_8: write-var mask <= s_33_7
        fn_state.mask = s_33_7;
        // N s_33_9: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // S s_34_1: call Bit(s_34_0)
        let s_34_1: bool = Bit(state, tracer, s_34_0);
        // C s_34_2: const #0s : i
        let s_34_2: i128 = 0;
        // D s_34_3: read-var mask:u8
        let s_34_3: u8 = fn_state.mask;
        // D s_34_4: cast zx s_34_3 -> bv
        let s_34_4: Bits = Bits::new(s_34_3 as u128, 3u16);
        // C s_34_5: const #1u : u64
        let s_34_5: u64 = 1;
        // D s_34_6: bit-insert s_34_4 s_34_4 s_34_2 s_34_5
        let s_34_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_34_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_34_4.length(),
            );
            (s_34_4 & mask) | (s_34_4 << s_34_2)
        };
        // D s_34_7: cast reint s_34_6 -> u8
        let s_34_7: u8 = (s_34_6.value() as u8);
        // D s_34_8: write-var mask <= s_34_7
        fn_state.mask = s_34_7;
        // N s_34_9: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_FMO(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_35_1);
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #0u : u8
        let s_35_4: bool = false;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // D s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // N s_35_7: branch s_35_6 b47 b36
        if s_35_6 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#327449 <= s_36_0
        fn_state.gs_327449 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_37_0: read-var gs#327449:u8
        let s_37_0: bool = fn_state.gs_327449;
        // N s_37_1: branch s_37_0 b46 b38
        if s_37_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_IMO(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #0u : u8
        let s_39_4: bool = false;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // N s_39_7: branch s_39_6 b45 b40
        if s_39_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#327450 <= s_40_0
        fn_state.gs_327450 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_41_0: read-var gs#327450:u8
        let s_41_0: bool = fn_state.gs_327450;
        // N s_41_1: branch s_41_0 b44 b42
        if s_41_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_42_0: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_43_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // S s_44_1: call Bit(s_44_0)
        let s_44_1: bool = Bit(state, tracer, s_44_0);
        // C s_44_2: const #1s : i
        let s_44_2: i128 = 1;
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
        // N s_44_9: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_45_0: const #90704u : u32
        let s_45_0: u32 = 90704;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_SCR_EL3_Type_IRQ(s_45_1)
        let s_45_2: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_45_1);
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #0u : u8
        let s_45_4: bool = false;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // D s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var gs#327450 <= s_45_6
        fn_state.gs_327450 = s_45_6;
        // N s_45_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // S s_46_1: call Bit(s_46_0)
        let s_46_1: bool = Bit(state, tracer, s_46_0);
        // C s_46_2: const #0s : i
        let s_46_2: i128 = 0;
        // D s_46_3: read-var mask:u8
        let s_46_3: u8 = fn_state.mask;
        // D s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 3u16);
        // C s_46_5: const #1u : u64
        let s_46_5: u64 = 1;
        // D s_46_6: bit-insert s_46_4 s_46_4 s_46_2 s_46_5
        let s_46_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_46_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_46_4.length(),
            );
            (s_46_4 & mask) | (s_46_4 << s_46_2)
        };
        // D s_46_7: cast reint s_46_6 -> u8
        let s_46_7: u8 = (s_46_6.value() as u8);
        // D s_46_8: write-var mask <= s_46_7
        fn_state.mask = s_46_7;
        // N s_46_9: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_47_0: const #90704u : u32
        let s_47_0: u32 = 90704;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_SCR_EL3_Type_FIQ(s_47_1)
        let s_47_2: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_47_1);
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #0u : u8
        let s_47_4: bool = false;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // D s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#327449 <= s_47_6
        fn_state.gs_327449 = s_47_6;
        // N s_47_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_48_0: read-var allintmask:u8
        let s_48_0: bool = fn_state.allintmask;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#327439 <= s_48_4
        fn_state.gs_327439 = s_48_4;
        // N s_48_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_49_0: const #16975u : u32
        let s_49_0: u32 = 16975;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: cast zx s_49_1 -> bv
        let s_49_2: Bits = Bits::new(s_49_1 as u128, 2u16);
        // C s_49_3: const #424u : u32
        let s_49_3: u32 = 424;
        // D s_49_4: read-reg s_49_3:u8
        let s_49_4: u8 = {
            let value = state.read_register::<u8>(s_49_3 as isize);
            tracer.read_register(s_49_3 as isize, value);
            value
        };
        // D s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 2u16);
        // D s_49_6: cmp-ne s_49_2 s_49_5
        let s_49_6: bool = ((s_49_2) != (s_49_5));
        // N s_49_7: branch s_49_6 b51 b50
        if s_49_6 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_50_0: const #90704u : u32
        let s_50_0: u32 = 90704;
        // D s_50_1: read-reg s_50_0:struct
        let s_50_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call _get_SCR_EL3_Type_EA(s_50_1)
        let s_50_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_50_1);
        // C s_50_3: const #90704u : u32
        let s_50_3: u32 = 90704;
        // D s_50_4: read-reg s_50_3:struct
        let s_50_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_3 as isize);
            tracer.read_register(s_50_3 as isize, value);
            value
        };
        // D s_50_5: call _get_SCR_EL3_Type_IRQ(s_50_4)
        let s_50_5: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_50_4);
        // C s_50_6: const #90704u : u32
        let s_50_6: u32 = 90704;
        // D s_50_7: read-reg s_50_6:struct
        let s_50_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_6 as isize);
            tracer.read_register(s_50_6 as isize, value);
            value
        };
        // D s_50_8: call _get_SCR_EL3_Type_FIQ(s_50_7)
        let s_50_8: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_50_7);
        // D s_50_9: cast zx s_50_5 -> bv
        let s_50_9: Bits = Bits::new(s_50_5 as u128, 1u16);
        // D s_50_10: cast zx s_50_8 -> bv
        let s_50_10: Bits = Bits::new(s_50_8 as u128, 1u16);
        // D s_50_11: cast reint s_50_9 -> u128
        let s_50_11: u128 = (s_50_9.value() as u128);
        // D s_50_12: size-of s_50_9
        let s_50_12: u16 = s_50_9.length();
        // D s_50_13: cast reint s_50_10 -> u128
        let s_50_13: u128 = (s_50_10.value() as u128);
        // D s_50_14: size-of s_50_10
        let s_50_14: u16 = s_50_10.length();
        // D s_50_15: lsl s_50_11 s_50_14
        let s_50_15: u128 = s_50_11 << s_50_14;
        // D s_50_16: or s_50_15 s_50_13
        let s_50_16: u128 = ((s_50_15) | (s_50_13));
        // D s_50_17: add s_50_12 s_50_14
        let s_50_17: u16 = (s_50_12 + s_50_14);
        // D s_50_18: create-bits s_50_16 s_50_17
        let s_50_18: Bits = Bits::new(s_50_16, s_50_17);
        // D s_50_19: cast reint s_50_18 -> u8
        let s_50_19: u8 = (s_50_18.value() as u8);
        // D s_50_20: cast zx s_50_2 -> bv
        let s_50_20: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_21: cast zx s_50_19 -> bv
        let s_50_21: Bits = Bits::new(s_50_19 as u128, 2u16);
        // D s_50_22: cast reint s_50_20 -> u128
        let s_50_22: u128 = (s_50_20.value() as u128);
        // D s_50_23: size-of s_50_20
        let s_50_23: u16 = s_50_20.length();
        // D s_50_24: cast reint s_50_21 -> u128
        let s_50_24: u128 = (s_50_21.value() as u128);
        // D s_50_25: size-of s_50_21
        let s_50_25: u16 = s_50_21.length();
        // D s_50_26: lsl s_50_22 s_50_25
        let s_50_26: u128 = s_50_22 << s_50_25;
        // D s_50_27: or s_50_26 s_50_24
        let s_50_27: u128 = ((s_50_26) | (s_50_24));
        // D s_50_28: add s_50_23 s_50_25
        let s_50_28: u16 = (s_50_23 + s_50_25);
        // D s_50_29: create-bits s_50_27 s_50_28
        let s_50_29: Bits = Bits::new(s_50_27, s_50_28);
        // D s_50_30: cast reint s_50_29 -> u8
        let s_50_30: u8 = (s_50_29.value() as u8);
        // D s_50_31: cast zx s_50_30 -> bv
        let s_50_31: Bits = Bits::new(s_50_30 as u128, 3u16);
        // D s_50_32: not s_50_31
        let s_50_32: Bits = !s_50_31;
        // D s_50_33: cast reint s_50_32 -> u8
        let s_50_33: u8 = (s_50_32.value() as u8);
        // D s_50_34: read-var mask:u8
        let s_50_34: u8 = fn_state.mask;
        // D s_50_35: cast zx s_50_34 -> bv
        let s_50_35: Bits = Bits::new(s_50_34 as u128, 3u16);
        // D s_50_36: cast zx s_50_33 -> bv
        let s_50_36: Bits = Bits::new(s_50_33 as u128, 3u16);
        // D s_50_37: or s_50_35 s_50_36
        let s_50_37: Bits = ((s_50_35) | (s_50_36));
        // D s_50_38: cast reint s_50_37 -> u8
        let s_50_38: u8 = (s_50_37.value() as u8);
        // D s_50_39: write-var mask <= s_50_38
        fn_state.mask = s_50_38;
        // N s_50_40: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_51_0: const #90704u : u32
        let s_51_0: u32 = 90704;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_SCR_EL3_Type_EA(s_51_1)
        let s_51_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_51_1);
        // C s_51_3: const #90704u : u32
        let s_51_3: u32 = 90704;
        // D s_51_4: read-reg s_51_3:struct
        let s_51_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_3 as isize);
            tracer.read_register(s_51_3 as isize, value);
            value
        };
        // D s_51_5: call _get_SCR_EL3_Type_IRQ(s_51_4)
        let s_51_5: bool = u_get_SCR_EL3_Type_IRQ(state, tracer, s_51_4);
        // C s_51_6: const #90704u : u32
        let s_51_6: u32 = 90704;
        // D s_51_7: read-reg s_51_6:struct
        let s_51_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_6 as isize);
            tracer.read_register(s_51_6 as isize, value);
            value
        };
        // D s_51_8: call _get_SCR_EL3_Type_FIQ(s_51_7)
        let s_51_8: bool = u_get_SCR_EL3_Type_FIQ(state, tracer, s_51_7);
        // D s_51_9: cast zx s_51_5 -> bv
        let s_51_9: Bits = Bits::new(s_51_5 as u128, 1u16);
        // D s_51_10: cast zx s_51_8 -> bv
        let s_51_10: Bits = Bits::new(s_51_8 as u128, 1u16);
        // D s_51_11: cast reint s_51_9 -> u128
        let s_51_11: u128 = (s_51_9.value() as u128);
        // D s_51_12: size-of s_51_9
        let s_51_12: u16 = s_51_9.length();
        // D s_51_13: cast reint s_51_10 -> u128
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
        // D s_51_20: cast zx s_51_2 -> bv
        let s_51_20: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_21: cast zx s_51_19 -> bv
        let s_51_21: Bits = Bits::new(s_51_19 as u128, 2u16);
        // D s_51_22: cast reint s_51_20 -> u128
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
        // D s_51_31: cast zx s_51_30 -> bv
        let s_51_31: Bits = Bits::new(s_51_30 as u128, 3u16);
        // D s_51_32: not s_51_31
        let s_51_32: Bits = !s_51_31;
        // D s_51_33: cast reint s_51_32 -> u8
        let s_51_33: u8 = (s_51_32.value() as u8);
        // D s_51_34: read-var mask:u8
        let s_51_34: u8 = fn_state.mask;
        // D s_51_35: cast zx s_51_34 -> bv
        let s_51_35: Bits = Bits::new(s_51_34 as u128, 3u16);
        // D s_51_36: cast zx s_51_33 -> bv
        let s_51_36: Bits = Bits::new(s_51_33 as u128, 3u16);
        // D s_51_37: and s_51_35 s_51_36
        let s_51_37: Bits = ((s_51_35) & (s_51_36));
        // D s_51_38: cast reint s_51_37 -> u8
        let s_51_38: u8 = (s_51_37.value() as u8);
        // D s_51_39: write-var mask <= s_51_38
        fn_state.mask = s_51_38;
        // N s_51_40: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call HaveVirtHostExt(s_52_0)
        let s_52_1: bool = HaveVirtHostExt(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b76 b53
        if s_52_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#327457 <= s_53_0
        fn_state.gs_327457 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_54_0: read-var gs#327457:u8
        let s_54_0: bool = fn_state.gs_327457;
        // N s_54_1: branch s_54_0 b75 b55
        if s_54_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#327458 <= s_55_0
        fn_state.gs_327458 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_56_0: read-var gs#327458:u8
        let s_56_0: bool = fn_state.gs_327458;
        // N s_56_1: branch s_56_0 b74 b57
        if s_56_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_57_0: const #102552u : u32
        let s_57_0: u32 = 102552;
        // D s_57_1: read-reg s_57_0:struct
        let s_57_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call _get_HCR_EL2_Type_TGE(s_57_1)
        let s_57_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_57_1);
        // D s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // D s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // N s_57_7: branch s_57_6 b73 b58
        if s_57_6 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_58_0: const #102552u : u32
        let s_58_0: u32 = 102552;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_HCR_EL2_Type_AMO(s_58_1)
        let s_58_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_58_1);
        // C s_58_3: const #102552u : u32
        let s_58_3: u32 = 102552;
        // D s_58_4: read-reg s_58_3:struct
        let s_58_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: call _get_HCR_EL2_Type_IMO(s_58_4)
        let s_58_5: bool = u_get_HCR_EL2_Type_IMO(state, tracer, s_58_4);
        // C s_58_6: const #102552u : u32
        let s_58_6: u32 = 102552;
        // D s_58_7: read-reg s_58_6:struct
        let s_58_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_6 as isize);
            tracer.read_register(s_58_6 as isize, value);
            value
        };
        // D s_58_8: call _get_HCR_EL2_Type_FMO(s_58_7)
        let s_58_8: bool = u_get_HCR_EL2_Type_FMO(state, tracer, s_58_7);
        // D s_58_9: cast zx s_58_5 -> bv
        let s_58_9: Bits = Bits::new(s_58_5 as u128, 1u16);
        // D s_58_10: cast zx s_58_8 -> bv
        let s_58_10: Bits = Bits::new(s_58_8 as u128, 1u16);
        // D s_58_11: cast reint s_58_9 -> u128
        let s_58_11: u128 = (s_58_9.value() as u128);
        // D s_58_12: size-of s_58_9
        let s_58_12: u16 = s_58_9.length();
        // D s_58_13: cast reint s_58_10 -> u128
        let s_58_13: u128 = (s_58_10.value() as u128);
        // D s_58_14: size-of s_58_10
        let s_58_14: u16 = s_58_10.length();
        // D s_58_15: lsl s_58_11 s_58_14
        let s_58_15: u128 = s_58_11 << s_58_14;
        // D s_58_16: or s_58_15 s_58_13
        let s_58_16: u128 = ((s_58_15) | (s_58_13));
        // D s_58_17: add s_58_12 s_58_14
        let s_58_17: u16 = (s_58_12 + s_58_14);
        // D s_58_18: create-bits s_58_16 s_58_17
        let s_58_18: Bits = Bits::new(s_58_16, s_58_17);
        // D s_58_19: cast reint s_58_18 -> u8
        let s_58_19: u8 = (s_58_18.value() as u8);
        // D s_58_20: cast zx s_58_2 -> bv
        let s_58_20: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_21: cast zx s_58_19 -> bv
        let s_58_21: Bits = Bits::new(s_58_19 as u128, 2u16);
        // D s_58_22: cast reint s_58_20 -> u128
        let s_58_22: u128 = (s_58_20.value() as u128);
        // D s_58_23: size-of s_58_20
        let s_58_23: u16 = s_58_20.length();
        // D s_58_24: cast reint s_58_21 -> u128
        let s_58_24: u128 = (s_58_21.value() as u128);
        // D s_58_25: size-of s_58_21
        let s_58_25: u16 = s_58_21.length();
        // D s_58_26: lsl s_58_22 s_58_25
        let s_58_26: u128 = s_58_22 << s_58_25;
        // D s_58_27: or s_58_26 s_58_24
        let s_58_27: u128 = ((s_58_26) | (s_58_24));
        // D s_58_28: add s_58_23 s_58_25
        let s_58_28: u16 = (s_58_23 + s_58_25);
        // D s_58_29: create-bits s_58_27 s_58_28
        let s_58_29: Bits = Bits::new(s_58_27, s_58_28);
        // D s_58_30: cast reint s_58_29 -> u8
        let s_58_30: u8 = (s_58_29.value() as u8);
        // D s_58_31: write-var mask_override <= s_58_30
        fn_state.mask_override = s_58_30;
        // N s_58_32: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_59_0: const #16975u : u32
        let s_59_0: u32 = 16975;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: cast zx s_59_1 -> bv
        let s_59_2: Bits = Bits::new(s_59_1 as u128, 2u16);
        // C s_59_3: const #448u : u32
        let s_59_3: u32 = 448;
        // D s_59_4: read-reg s_59_3:u8
        let s_59_4: u8 = {
            let value = state.read_register::<u8>(s_59_3 as isize);
            tracer.read_register(s_59_3 as isize, value);
            value
        };
        // D s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 2u16);
        // D s_59_6: cmp-eq s_59_2 s_59_5
        let s_59_6: bool = ((s_59_2) == (s_59_5));
        // N s_59_7: branch s_59_6 b72 b60
        if s_59_6 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_60_0: const #16975u : u32
        let s_60_0: u32 = 16975;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: cast zx s_60_1 -> bv
        let s_60_2: Bits = Bits::new(s_60_1 as u128, 2u16);
        // C s_60_3: const #440u : u32
        let s_60_3: u32 = 440;
        // D s_60_4: read-reg s_60_3:u8
        let s_60_4: u8 = {
            let value = state.read_register::<u8>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: cast zx s_60_4 -> bv
        let s_60_5: Bits = Bits::new(s_60_4 as u128, 2u16);
        // D s_60_6: cmp-eq s_60_2 s_60_5
        let s_60_6: bool = ((s_60_2) == (s_60_5));
        // D s_60_7: write-var gs#327460 <= s_60_6
        fn_state.gs_327460 = s_60_6;
        // N s_60_8: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_61_0: read-var gs#327460:u8
        let s_61_0: bool = fn_state.gs_327460;
        // N s_61_1: branch s_61_0 b71 b62
        if s_61_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_62_0: const #16975u : u32
        let s_62_0: u32 = 16975;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: cast zx s_62_1 -> bv
        let s_62_2: Bits = Bits::new(s_62_1 as u128, 2u16);
        // C s_62_3: const #432u : u32
        let s_62_3: u32 = 432;
        // D s_62_4: read-reg s_62_3:u8
        let s_62_4: u8 = {
            let value = state.read_register::<u8>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 2u16);
        // D s_62_6: cmp-eq s_62_2 s_62_5
        let s_62_6: bool = ((s_62_2) == (s_62_5));
        // N s_62_7: branch s_62_6 b64 b63
        if s_62_6 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_63_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_64_0: const #432u : u32
        let s_64_0: u32 = 432;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call ELUsingAArch32(s_64_1)
        let s_64_2: bool = ELUsingAArch32(state, tracer, s_64_1);
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // N s_64_4: branch s_64_3 b70 b65
        if s_64_3 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#327461 <= s_65_0
        fn_state.gs_327461 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_66_0: read-var gs#327461:u8
        let s_66_0: bool = fn_state.gs_327461;
        // N s_66_1: branch s_66_0 b69 b67
        if s_66_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_68_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_69_0: read-var mask_override:u8
        let s_69_0: u8 = fn_state.mask_override;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 3u16);
        // D s_69_2: not s_69_1
        let s_69_2: Bits = !s_69_1;
        // D s_69_3: cast reint s_69_2 -> u8
        let s_69_3: u8 = (s_69_2.value() as u8);
        // D s_69_4: read-var mask:u8
        let s_69_4: u8 = fn_state.mask;
        // D s_69_5: cast zx s_69_4 -> bv
        let s_69_5: Bits = Bits::new(s_69_4 as u128, 3u16);
        // D s_69_6: cast zx s_69_3 -> bv
        let s_69_6: Bits = Bits::new(s_69_3 as u128, 3u16);
        // D s_69_7: or s_69_5 s_69_6
        let s_69_7: Bits = ((s_69_5) | (s_69_6));
        // D s_69_8: cast reint s_69_7 -> u8
        let s_69_8: u8 = (s_69_7.value() as u8);
        // D s_69_9: write-var mask <= s_69_8
        fn_state.mask = s_69_8;
        // N s_69_10: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_70_0: const #102552u : u32
        let s_70_0: u32 = 102552;
        // D s_70_1: read-reg s_70_0:struct
        let s_70_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call _get_HCR_EL2_Type_TGE(s_70_1)
        let s_70_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_70_1);
        // D s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // C s_70_4: const #0u : u8
        let s_70_4: bool = false;
        // C s_70_5: cast zx s_70_4 -> bv
        let s_70_5: Bits = Bits::new(s_70_4 as u128, 1u16);
        // D s_70_6: cmp-eq s_70_3 s_70_5
        let s_70_6: bool = ((s_70_3) == (s_70_5));
        // D s_70_7: write-var gs#327461 <= s_70_6
        fn_state.gs_327461 = s_70_6;
        // N s_70_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_71_0: read-var mask_override:u8
        let s_71_0: u8 = fn_state.mask_override;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 3u16);
        // D s_71_2: not s_71_1
        let s_71_2: Bits = !s_71_1;
        // D s_71_3: cast reint s_71_2 -> u8
        let s_71_3: u8 = (s_71_2.value() as u8);
        // D s_71_4: read-var mask:u8
        let s_71_4: u8 = fn_state.mask;
        // D s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 3u16);
        // D s_71_6: cast zx s_71_3 -> bv
        let s_71_6: Bits = Bits::new(s_71_3 as u128, 3u16);
        // D s_71_7: and s_71_5 s_71_6
        let s_71_7: Bits = ((s_71_5) & (s_71_6));
        // D s_71_8: cast reint s_71_7 -> u8
        let s_71_8: u8 = (s_71_7.value() as u8);
        // D s_71_9: write-var mask <= s_71_8
        fn_state.mask = s_71_8;
        // N s_71_10: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#327460 <= s_72_0
        fn_state.gs_327460 = s_72_0;
        // N s_72_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_73_0: const #7u : u8
        let s_73_0: u8 = 7;
        // D s_73_1: write-var mask_override <= s_73_0
        fn_state.mask_override = s_73_0;
        // N s_73_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_74_0: const #0u : u8
        let s_74_0: u8 = 0;
        // D s_74_1: write-var mask_override <= s_74_0
        fn_state.mask_override = s_74_0;
        // N s_74_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_75_0: const #102552u : u32
        let s_75_0: u32 = 102552;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call _get_HCR_EL2_Type_TGE(s_75_1)
        let s_75_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_75_1);
        // D s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // C s_75_4: const #1u : u8
        let s_75_4: bool = true;
        // C s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 1u16);
        // D s_75_6: cmp-eq s_75_3 s_75_5
        let s_75_6: bool = ((s_75_3) == (s_75_5));
        // D s_75_7: write-var gs#327458 <= s_75_6
        fn_state.gs_327458 = s_75_6;
        // N s_75_8: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_76_0: const #102552u : u32
        let s_76_0: u32 = 102552;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_HCR_EL2_Type_E2H(s_76_1)
        let s_76_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_76_1);
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // C s_76_4: const #1u : u8
        let s_76_4: bool = true;
        // C s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 1u16);
        // D s_76_6: cmp-eq s_76_3 s_76_5
        let s_76_6: bool = ((s_76_3) == (s_76_5));
        // D s_76_7: write-var gs#327457 <= s_76_6
        fn_state.gs_327457 = s_76_6;
        // N s_76_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_77_0: const #16969u : u32
        let s_77_0: u32 = 16969;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: bool = {
            let value = state.read_register::<bool>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // C s_77_2: const #16990u : u32
        let s_77_2: u32 = 16990;
        // D s_77_3: read-reg s_77_2:u8
        let s_77_3: bool = {
            let value = state.read_register::<bool>(s_77_2 as isize);
            tracer.read_register(s_77_2 as isize, value);
            value
        };
        // C s_77_4: const #() : ()
        let s_77_4: () = ();
        // S s_77_5: call SCTLR_read__1(s_77_4)
        let s_77_5: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_77_4);
        // S s_77_6: call _get_SCTLRType_SPINTMASK(s_77_5)
        let s_77_6: bool = u_get_SCTLRType_SPINTMASK(state, tracer, s_77_5);
        // D s_77_7: cast zx s_77_3 -> bv
        let s_77_7: Bits = Bits::new(s_77_3 as u128, 1u16);
        // S s_77_8: cast zx s_77_6 -> bv
        let s_77_8: Bits = Bits::new(s_77_6 as u128, 1u16);
        // D s_77_9: and s_77_7 s_77_8
        let s_77_9: Bits = ((s_77_7) & (s_77_8));
        // D s_77_10: cast reint s_77_9 -> u8
        let s_77_10: bool = ((s_77_9.value()) != 0);
        // D s_77_11: cast zx s_77_1 -> bv
        let s_77_11: Bits = Bits::new(s_77_1 as u128, 1u16);
        // D s_77_12: cast zx s_77_10 -> bv
        let s_77_12: Bits = Bits::new(s_77_10 as u128, 1u16);
        // D s_77_13: or s_77_11 s_77_12
        let s_77_13: Bits = ((s_77_11) | (s_77_12));
        // D s_77_14: cast reint s_77_13 -> u8
        let s_77_14: bool = ((s_77_13.value()) != 0);
        // D s_77_15: write-var allintmask <= s_77_14
        fn_state.allintmask = s_77_14;
        // C s_77_16: const #16975u : u32
        let s_77_16: u32 = 16975;
        // D s_77_17: read-reg s_77_16:u8
        let s_77_17: u8 = {
            let value = state.read_register::<u8>(s_77_16 as isize);
            tracer.read_register(s_77_16 as isize, value);
            value
        };
        // D s_77_18: cast zx s_77_17 -> bv
        let s_77_18: Bits = Bits::new(s_77_17 as u128, 2u16);
        // C s_77_19: const #448u : u32
        let s_77_19: u32 = 448;
        // D s_77_20: read-reg s_77_19:u8
        let s_77_20: u8 = {
            let value = state.read_register::<u8>(s_77_19 as isize);
            tracer.read_register(s_77_19 as isize, value);
            value
        };
        // D s_77_21: cast zx s_77_20 -> bv
        let s_77_21: Bits = Bits::new(s_77_20 as u128, 2u16);
        // D s_77_22: cmp-eq s_77_18 s_77_21
        let s_77_22: bool = ((s_77_18) == (s_77_21));
        // N s_77_23: branch s_77_22 b89 b78
        if s_77_22 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_78_0: read-var allintmask:u8
        let s_78_0: bool = fn_state.allintmask;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#327432 <= s_78_4
        fn_state.gs_327432 = s_78_4;
        // N s_78_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_79_0: read-var gs#327432:u8
        let s_79_0: bool = fn_state.gs_327432;
        // N s_79_1: branch s_79_0 b82 b80
        if s_79_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_80_0: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_81_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_82_0: read-var irq_nmi:u8
        let s_82_0: bool = fn_state.irq_nmi;
        // N s_82_1: branch s_82_0 b88 b83
        if s_82_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_83_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_84_0: read-var fiq_nmi:u8
        let s_84_0: bool = fn_state.fiq_nmi;
        // N s_84_1: branch s_84_0 b87 b85
        if s_84_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_85_0: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // N s_86_0: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // S s_87_1: call Bit(s_87_0)
        let s_87_1: bool = Bit(state, tracer, s_87_0);
        // C s_87_2: const #0s : i
        let s_87_2: i128 = 0;
        // D s_87_3: read-var mask:u8
        let s_87_3: u8 = fn_state.mask;
        // D s_87_4: cast zx s_87_3 -> bv
        let s_87_4: Bits = Bits::new(s_87_3 as u128, 3u16);
        // C s_87_5: const #1u : u64
        let s_87_5: u64 = 1;
        // D s_87_6: bit-insert s_87_4 s_87_4 s_87_2 s_87_5
        let s_87_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_87_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_87_4.length(),
            );
            (s_87_4 & mask) | (s_87_4 << s_87_2)
        };
        // D s_87_7: cast reint s_87_6 -> u8
        let s_87_7: u8 = (s_87_6.value() as u8);
        // D s_87_8: write-var mask <= s_87_7
        fn_state.mask = s_87_7;
        // N s_87_9: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // S s_88_1: call Bit(s_88_0)
        let s_88_1: bool = Bit(state, tracer, s_88_0);
        // C s_88_2: const #1s : i
        let s_88_2: i128 = 1;
        // D s_88_3: read-var mask:u8
        let s_88_3: u8 = fn_state.mask;
        // D s_88_4: cast zx s_88_3 -> bv
        let s_88_4: Bits = Bits::new(s_88_3 as u128, 3u16);
        // C s_88_5: const #1u : u64
        let s_88_5: u64 = 1;
        // D s_88_6: bit-insert s_88_4 s_88_4 s_88_2 s_88_5
        let s_88_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_88_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_88_4.length(),
            );
            (s_88_4 & mask) | (s_88_4 << s_88_2)
        };
        // D s_88_7: cast reint s_88_6 -> u8
        let s_88_7: u8 = (s_88_6.value() as u8);
        // D s_88_8: write-var mask <= s_88_7
        fn_state.mask = s_88_7;
        // N s_88_9: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#327432 <= s_89_0
        fn_state.gs_327432 = s_89_0;
        // N s_89_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call SCTLR_read__1(s_90_0)
        let s_90_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_90_0);
        // S s_90_2: call _get_SCTLRType_NMI(s_90_1)
        let s_90_2: bool = u_get_SCTLRType_NMI(state, tracer, s_90_1);
        // S s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // C s_90_4: const #1u : u8
        let s_90_4: bool = true;
        // C s_90_5: cast zx s_90_4 -> bv
        let s_90_5: Bits = Bits::new(s_90_4 as u128, 1u16);
        // S s_90_6: cmp-eq s_90_3 s_90_5
        let s_90_6: bool = ((s_90_3) == (s_90_5));
        // D s_90_7: write-var gs#327430 <= s_90_6
        fn_state.gs_327430 = s_90_6;
        // N s_90_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_91_0: const #1u : u8
        let s_91_0: bool = true;
        // D s_91_1: write-var fiq_pending <= s_91_0
        fn_state.fiq_pending = s_91_0;
        // N s_91_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var irq_pending <= s_92_0
        fn_state.irq_pending = s_92_0;
        // N s_92_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var se_pending <= s_93_0
        fn_state.se_pending = s_93_0;
        // N s_93_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
