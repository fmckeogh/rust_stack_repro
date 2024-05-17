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
use u_get_HCR_Type_TGE::*;
use EL2Enabled::*;
use AArch64_PendingUnmaskedVirtualInterrupts::*;
use HCR_read::*;
use u_get_HCR_Type_FMO::*;
use ELUsingAArch32::*;
use u_get_HCR_Type_VF::*;
use u_get_HCR_Type_IMO::*;
use u_get_HCR_Type_VA::*;
use u_get_HCR_Type_VI::*;
use u_get_HCR_Type_AMO::*;
use common::*;
pub fn AArch32_PendingUnmaskedVirtualInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_327530: (),
) -> ProductTyped8f896a024a4e2cb {
    #[derive(Default)]
    struct FunctionState {
        gs_327535: bool,
        gs_327536: bool,
        gs_327537: bool,
        gs_327531: bool,
        gs_327533: bool,
        pending: u8,
        gs_327532: bool,
        mask: u8,
        return_value: ProductTyped8f896a024a4e2cb,
        gs_327530: (),
    }
    let fn_state = FunctionState {
        gs_327530,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
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
        // N s_0_4: branch s_0_3 b24 b1
        if s_0_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#327531 <= s_1_0
        fn_state.gs_327531 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_2_0: read-var gs#327531:u8
        let s_2_0: bool = fn_state.gs_327531;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // D s_3_3: cmp-lt s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) < (s_3_2));
        // N s_3_4: branch s_3_3 b22 b4
        if s_3_3 {
            return block_22(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#327532 <= s_4_0
        fn_state.gs_327532 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_5_0: read-var gs#327532:u8
        let s_5_0: bool = fn_state.gs_327532;
        // D s_5_1: write-var gs#327533 <= s_5_0
        fn_state.gs_327533 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_6_0: read-var gs#327533:u8
        let s_6_0: bool = fn_state.gs_327533;
        // N s_6_1: branch s_6_0 b21 b7
        if s_6_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_7_0: const #16968u : u32
        let s_7_0: u32 = 16968;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: bool = {
            let value = state.read_register::<bool>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #16979u : u32
        let s_7_2: u32 = 16979;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: bool = {
            let value = state.read_register::<bool>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // C s_7_4: const #16977u : u32
        let s_7_4: u32 = 16977;
        // D s_7_5: read-reg s_7_4:u8
        let s_7_5: bool = {
            let value = state.read_register::<bool>(s_7_4 as isize);
            tracer.read_register(s_7_4 as isize, value);
            value
        };
        // D s_7_6: cast zx s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 1u16);
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 1u16);
        // D s_7_8: cast reint s_7_6 -> u128
        let s_7_8: u128 = (s_7_6.value() as u128);
        // D s_7_9: size-of s_7_6
        let s_7_9: u16 = s_7_6.length();
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: lsl s_7_8 s_7_11
        let s_7_12: u128 = s_7_8 << s_7_11;
        // D s_7_13: or s_7_12 s_7_10
        let s_7_13: u128 = ((s_7_12) | (s_7_10));
        // D s_7_14: add s_7_9 s_7_11
        let s_7_14: u16 = (s_7_9 + s_7_11);
        // D s_7_15: create-bits s_7_13 s_7_14
        let s_7_15: Bits = Bits::new(s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u8
        let s_7_16: u8 = (s_7_15.value() as u8);
        // D s_7_17: cast zx s_7_1 -> bv
        let s_7_17: Bits = Bits::new(s_7_1 as u128, 1u16);
        // D s_7_18: cast zx s_7_16 -> bv
        let s_7_18: Bits = Bits::new(s_7_16 as u128, 2u16);
        // D s_7_19: cast reint s_7_17 -> u128
        let s_7_19: u128 = (s_7_17.value() as u128);
        // D s_7_20: size-of s_7_17
        let s_7_20: u16 = s_7_17.length();
        // D s_7_21: cast reint s_7_18 -> u128
        let s_7_21: u128 = (s_7_18.value() as u128);
        // D s_7_22: size-of s_7_18
        let s_7_22: u16 = s_7_18.length();
        // D s_7_23: lsl s_7_19 s_7_22
        let s_7_23: u128 = s_7_19 << s_7_22;
        // D s_7_24: or s_7_23 s_7_21
        let s_7_24: u128 = ((s_7_23) | (s_7_21));
        // D s_7_25: add s_7_20 s_7_22
        let s_7_25: u16 = (s_7_20 + s_7_22);
        // D s_7_26: create-bits s_7_24 s_7_25
        let s_7_26: Bits = Bits::new(s_7_24, s_7_25);
        // D s_7_27: cast reint s_7_26 -> u8
        let s_7_27: u8 = (s_7_26.value() as u8);
        // D s_7_28: write-var mask <= s_7_27
        fn_state.mask = s_7_27;
        // C s_7_29: const #16975u : u32
        let s_7_29: u32 = 16975;
        // D s_7_30: read-reg s_7_29:u8
        let s_7_30: u8 = {
            let value = state.read_register::<u8>(s_7_29 as isize);
            tracer.read_register(s_7_29 as isize, value);
            value
        };
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 2u16);
        // C s_7_32: const #448u : u32
        let s_7_32: u32 = 448;
        // D s_7_33: read-reg s_7_32:u8
        let s_7_33: u8 = {
            let value = state.read_register::<u8>(s_7_32 as isize);
            tracer.read_register(s_7_32 as isize, value);
            value
        };
        // D s_7_34: cast zx s_7_33 -> bv
        let s_7_34: Bits = Bits::new(s_7_33 as u128, 2u16);
        // D s_7_35: cmp-eq s_7_31 s_7_34
        let s_7_35: bool = ((s_7_31) == (s_7_34));
        // N s_7_36: branch s_7_35 b20 b8
        if s_7_35 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #440u : u32
        let s_8_3: u32 = 440;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // D s_8_7: write-var gs#327535 <= s_8_6
        fn_state.gs_327535 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_9_0: read-var gs#327535:u8
        let s_9_0: bool = fn_state.gs_327535;
        // N s_9_1: branch s_9_0 b19 b10
        if s_9_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#327536 <= s_10_0
        fn_state.gs_327536 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_11_0: read-var gs#327536:u8
        let s_11_0: bool = fn_state.gs_327536;
        // N s_11_1: branch s_11_0 b18 b12
        if s_11_0 {
            return block_18(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#327537 <= s_12_0
        fn_state.gs_327537 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_13_0: read-var gs#327537:u8
        let s_13_0: bool = fn_state.gs_327537;
        // N s_13_1: branch s_13_0 b17 b14
        if s_13_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_14_0: const #0u : u8
        let s_14_0: u8 = 0;
        // D s_14_1: write-var pending <= s_14_0
        fn_state.pending = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_15_0: read-var mask:u8
        let s_15_0: u8 = fn_state.mask;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 3u16);
        // D s_15_2: not s_15_1
        let s_15_2: Bits = !s_15_1;
        // D s_15_3: cast reint s_15_2 -> u8
        let s_15_3: u8 = (s_15_2.value() as u8);
        // D s_15_4: read-var pending:u8
        let s_15_4: u8 = fn_state.pending;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 3u16);
        // D s_15_6: cast zx s_15_3 -> bv
        let s_15_6: Bits = Bits::new(s_15_3 as u128, 3u16);
        // D s_15_7: and s_15_5 s_15_6
        let s_15_7: Bits = ((s_15_5) & (s_15_6));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // C s_15_9: const #2s : i
        let s_15_9: i128 = 2;
        // D s_15_10: cast zx s_15_8 -> bv
        let s_15_10: Bits = Bits::new(s_15_8 as u128, 3u16);
        // C s_15_11: const #1u : u64
        let s_15_11: u64 = 1;
        // D s_15_12: bit-extract s_15_10 s_15_9 s_15_11
        let s_15_12: Bits = (Bits::new(
            ((s_15_10) >> (s_15_9)).value(),
            u16::try_from(s_15_11).unwrap(),
        ));
        // D s_15_13: cast reint s_15_12 -> u8
        let s_15_13: bool = ((s_15_12.value()) != 0);
        // C s_15_14: const #0s : i
        let s_15_14: i128 = 0;
        // C s_15_15: const #0u : u64
        let s_15_15: u64 = 0;
        // D s_15_16: cast zx s_15_13 -> u64
        let s_15_16: u64 = (s_15_13 as u64);
        // C s_15_17: const #1u : u64
        let s_15_17: u64 = 1;
        // D s_15_18: and s_15_16 s_15_17
        let s_15_18: u64 = ((s_15_16) & (s_15_17));
        // D s_15_19: cmp-eq s_15_18 s_15_17
        let s_15_19: bool = ((s_15_18) == (s_15_17));
        // D s_15_20: lsl s_15_16 s_15_14
        let s_15_20: u64 = s_15_16 << s_15_14;
        // D s_15_21: or s_15_15 s_15_20
        let s_15_21: u64 = ((s_15_15) | (s_15_20));
        // D s_15_22: cmpl s_15_20
        let s_15_22: u64 = !s_15_20;
        // D s_15_23: and s_15_15 s_15_22
        let s_15_23: u64 = ((s_15_15) & (s_15_22));
        // D s_15_24: select s_15_19 s_15_21 s_15_23
        let s_15_24: u64 = if s_15_19 { s_15_21 } else { s_15_23 };
        // D s_15_25: cast trunc s_15_24 -> u8
        let s_15_25: bool = ((s_15_24) != 0);
        // D s_15_26: cast zx s_15_25 -> bv
        let s_15_26: Bits = Bits::new(s_15_25 as u128, 1u16);
        // C s_15_27: const #1u : u8
        let s_15_27: bool = true;
        // C s_15_28: cast zx s_15_27 -> bv
        let s_15_28: Bits = Bits::new(s_15_27 as u128, 1u16);
        // D s_15_29: cmp-eq s_15_26 s_15_28
        let s_15_29: bool = ((s_15_26) == (s_15_28));
        // C s_15_30: const #1s : i
        let s_15_30: i128 = 1;
        // D s_15_31: cast zx s_15_8 -> bv
        let s_15_31: Bits = Bits::new(s_15_8 as u128, 3u16);
        // C s_15_32: const #1u : u64
        let s_15_32: u64 = 1;
        // D s_15_33: bit-extract s_15_31 s_15_30 s_15_32
        let s_15_33: Bits = (Bits::new(
            ((s_15_31) >> (s_15_30)).value(),
            u16::try_from(s_15_32).unwrap(),
        ));
        // D s_15_34: cast reint s_15_33 -> u8
        let s_15_34: bool = ((s_15_33.value()) != 0);
        // C s_15_35: const #0s : i
        let s_15_35: i128 = 0;
        // C s_15_36: const #0u : u64
        let s_15_36: u64 = 0;
        // D s_15_37: cast zx s_15_34 -> u64
        let s_15_37: u64 = (s_15_34 as u64);
        // C s_15_38: const #1u : u64
        let s_15_38: u64 = 1;
        // D s_15_39: and s_15_37 s_15_38
        let s_15_39: u64 = ((s_15_37) & (s_15_38));
        // D s_15_40: cmp-eq s_15_39 s_15_38
        let s_15_40: bool = ((s_15_39) == (s_15_38));
        // D s_15_41: lsl s_15_37 s_15_35
        let s_15_41: u64 = s_15_37 << s_15_35;
        // D s_15_42: or s_15_36 s_15_41
        let s_15_42: u64 = ((s_15_36) | (s_15_41));
        // D s_15_43: cmpl s_15_41
        let s_15_43: u64 = !s_15_41;
        // D s_15_44: and s_15_36 s_15_43
        let s_15_44: u64 = ((s_15_36) & (s_15_43));
        // D s_15_45: select s_15_40 s_15_42 s_15_44
        let s_15_45: u64 = if s_15_40 { s_15_42 } else { s_15_44 };
        // D s_15_46: cast trunc s_15_45 -> u8
        let s_15_46: bool = ((s_15_45) != 0);
        // D s_15_47: cast zx s_15_46 -> bv
        let s_15_47: Bits = Bits::new(s_15_46 as u128, 1u16);
        // C s_15_48: const #1u : u8
        let s_15_48: bool = true;
        // C s_15_49: cast zx s_15_48 -> bv
        let s_15_49: Bits = Bits::new(s_15_48 as u128, 1u16);
        // D s_15_50: cmp-eq s_15_47 s_15_49
        let s_15_50: bool = ((s_15_47) == (s_15_49));
        // C s_15_51: const #0s : i
        let s_15_51: i128 = 0;
        // D s_15_52: cast zx s_15_8 -> bv
        let s_15_52: Bits = Bits::new(s_15_8 as u128, 3u16);
        // C s_15_53: const #1u : u64
        let s_15_53: u64 = 1;
        // D s_15_54: bit-extract s_15_52 s_15_51 s_15_53
        let s_15_54: Bits = (Bits::new(
            ((s_15_52) >> (s_15_51)).value(),
            u16::try_from(s_15_53).unwrap(),
        ));
        // D s_15_55: cast reint s_15_54 -> u8
        let s_15_55: bool = ((s_15_54.value()) != 0);
        // C s_15_56: const #0s : i
        let s_15_56: i128 = 0;
        // C s_15_57: const #0u : u64
        let s_15_57: u64 = 0;
        // D s_15_58: cast zx s_15_55 -> u64
        let s_15_58: u64 = (s_15_55 as u64);
        // C s_15_59: const #1u : u64
        let s_15_59: u64 = 1;
        // D s_15_60: and s_15_58 s_15_59
        let s_15_60: u64 = ((s_15_58) & (s_15_59));
        // D s_15_61: cmp-eq s_15_60 s_15_59
        let s_15_61: bool = ((s_15_60) == (s_15_59));
        // D s_15_62: lsl s_15_58 s_15_56
        let s_15_62: u64 = s_15_58 << s_15_56;
        // D s_15_63: or s_15_57 s_15_62
        let s_15_63: u64 = ((s_15_57) | (s_15_62));
        // D s_15_64: cmpl s_15_62
        let s_15_64: u64 = !s_15_62;
        // D s_15_65: and s_15_57 s_15_64
        let s_15_65: u64 = ((s_15_57) & (s_15_64));
        // D s_15_66: select s_15_61 s_15_63 s_15_65
        let s_15_66: u64 = if s_15_61 { s_15_63 } else { s_15_65 };
        // D s_15_67: cast trunc s_15_66 -> u8
        let s_15_67: bool = ((s_15_66) != 0);
        // D s_15_68: cast zx s_15_67 -> bv
        let s_15_68: Bits = Bits::new(s_15_67 as u128, 1u16);
        // C s_15_69: const #1u : u8
        let s_15_69: bool = true;
        // C s_15_70: cast zx s_15_69 -> bv
        let s_15_70: Bits = Bits::new(s_15_69 as u128, 1u16);
        // D s_15_71: cmp-eq s_15_68 s_15_70
        let s_15_71: bool = ((s_15_68) == (s_15_70));
        // D s_15_72: create-product struct = ["s_15_29", "s_15_50", "s_15_71"]
        let s_15_72: ProductTyped8f896a024a4e2cb = ProductTyped8f896a024a4e2cb {
            _0: s_15_29,
            _1: s_15_50,
            _2: s_15_71,
        };
        // D s_15_73: write-var return_value <= s_15_72
        fn_state.return_value = s_15_72;
        // N s_15_74: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // D s_16_0: read-var return_value:struct
        let s_16_0: ProductTyped8f896a024a4e2cb = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HCR_read(s_17_0)
        let s_17_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_0);
        // S s_17_2: call _get_HCR_Type_VA(s_17_1)
        let s_17_2: bool = u_get_HCR_Type_VA(state, tracer, s_17_1);
        // C s_17_3: const #() : ()
        let s_17_3: () = ();
        // S s_17_4: call HCR_read(s_17_3)
        let s_17_4: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_3);
        // S s_17_5: call _get_HCR_Type_VI(s_17_4)
        let s_17_5: bool = u_get_HCR_Type_VI(state, tracer, s_17_4);
        // C s_17_6: const #() : ()
        let s_17_6: () = ();
        // S s_17_7: call HCR_read(s_17_6)
        let s_17_7: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_6);
        // S s_17_8: call _get_HCR_Type_VF(s_17_7)
        let s_17_8: bool = u_get_HCR_Type_VF(state, tracer, s_17_7);
        // S s_17_9: cast zx s_17_5 -> bv
        let s_17_9: Bits = Bits::new(s_17_5 as u128, 1u16);
        // S s_17_10: cast zx s_17_8 -> bv
        let s_17_10: Bits = Bits::new(s_17_8 as u128, 1u16);
        // S s_17_11: cast reint s_17_9 -> u128
        let s_17_11: u128 = (s_17_9.value() as u128);
        // D s_17_12: size-of s_17_9
        let s_17_12: u16 = s_17_9.length();
        // S s_17_13: cast reint s_17_10 -> u128
        let s_17_13: u128 = (s_17_10.value() as u128);
        // D s_17_14: size-of s_17_10
        let s_17_14: u16 = s_17_10.length();
        // D s_17_15: lsl s_17_11 s_17_14
        let s_17_15: u128 = s_17_11 << s_17_14;
        // D s_17_16: or s_17_15 s_17_13
        let s_17_16: u128 = ((s_17_15) | (s_17_13));
        // D s_17_17: add s_17_12 s_17_14
        let s_17_17: u16 = (s_17_12 + s_17_14);
        // D s_17_18: create-bits s_17_16 s_17_17
        let s_17_18: Bits = Bits::new(s_17_16, s_17_17);
        // D s_17_19: cast reint s_17_18 -> u8
        let s_17_19: u8 = (s_17_18.value() as u8);
        // S s_17_20: cast zx s_17_2 -> bv
        let s_17_20: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_21: cast zx s_17_19 -> bv
        let s_17_21: Bits = Bits::new(s_17_19 as u128, 2u16);
        // S s_17_22: cast reint s_17_20 -> u128
        let s_17_22: u128 = (s_17_20.value() as u128);
        // D s_17_23: size-of s_17_20
        let s_17_23: u16 = s_17_20.length();
        // D s_17_24: cast reint s_17_21 -> u128
        let s_17_24: u128 = (s_17_21.value() as u128);
        // D s_17_25: size-of s_17_21
        let s_17_25: u16 = s_17_21.length();
        // D s_17_26: lsl s_17_22 s_17_25
        let s_17_26: u128 = s_17_22 << s_17_25;
        // D s_17_27: or s_17_26 s_17_24
        let s_17_27: u128 = ((s_17_26) | (s_17_24));
        // D s_17_28: add s_17_23 s_17_25
        let s_17_28: u16 = (s_17_23 + s_17_25);
        // D s_17_29: create-bits s_17_27 s_17_28
        let s_17_29: Bits = Bits::new(s_17_27, s_17_28);
        // D s_17_30: cast reint s_17_29 -> u8
        let s_17_30: u8 = (s_17_29.value() as u8);
        // C s_17_31: const #() : ()
        let s_17_31: () = ();
        // S s_17_32: call HCR_read(s_17_31)
        let s_17_32: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_31);
        // S s_17_33: call _get_HCR_Type_AMO(s_17_32)
        let s_17_33: bool = u_get_HCR_Type_AMO(state, tracer, s_17_32);
        // C s_17_34: const #() : ()
        let s_17_34: () = ();
        // S s_17_35: call HCR_read(s_17_34)
        let s_17_35: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_34);
        // S s_17_36: call _get_HCR_Type_IMO(s_17_35)
        let s_17_36: bool = u_get_HCR_Type_IMO(state, tracer, s_17_35);
        // C s_17_37: const #() : ()
        let s_17_37: () = ();
        // S s_17_38: call HCR_read(s_17_37)
        let s_17_38: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_37);
        // S s_17_39: call _get_HCR_Type_FMO(s_17_38)
        let s_17_39: bool = u_get_HCR_Type_FMO(state, tracer, s_17_38);
        // S s_17_40: cast zx s_17_36 -> bv
        let s_17_40: Bits = Bits::new(s_17_36 as u128, 1u16);
        // S s_17_41: cast zx s_17_39 -> bv
        let s_17_41: Bits = Bits::new(s_17_39 as u128, 1u16);
        // S s_17_42: cast reint s_17_40 -> u128
        let s_17_42: u128 = (s_17_40.value() as u128);
        // D s_17_43: size-of s_17_40
        let s_17_43: u16 = s_17_40.length();
        // S s_17_44: cast reint s_17_41 -> u128
        let s_17_44: u128 = (s_17_41.value() as u128);
        // D s_17_45: size-of s_17_41
        let s_17_45: u16 = s_17_41.length();
        // D s_17_46: lsl s_17_42 s_17_45
        let s_17_46: u128 = s_17_42 << s_17_45;
        // D s_17_47: or s_17_46 s_17_44
        let s_17_47: u128 = ((s_17_46) | (s_17_44));
        // D s_17_48: add s_17_43 s_17_45
        let s_17_48: u16 = (s_17_43 + s_17_45);
        // D s_17_49: create-bits s_17_47 s_17_48
        let s_17_49: Bits = Bits::new(s_17_47, s_17_48);
        // D s_17_50: cast reint s_17_49 -> u8
        let s_17_50: u8 = (s_17_49.value() as u8);
        // S s_17_51: cast zx s_17_33 -> bv
        let s_17_51: Bits = Bits::new(s_17_33 as u128, 1u16);
        // D s_17_52: cast zx s_17_50 -> bv
        let s_17_52: Bits = Bits::new(s_17_50 as u128, 2u16);
        // S s_17_53: cast reint s_17_51 -> u128
        let s_17_53: u128 = (s_17_51.value() as u128);
        // D s_17_54: size-of s_17_51
        let s_17_54: u16 = s_17_51.length();
        // D s_17_55: cast reint s_17_52 -> u128
        let s_17_55: u128 = (s_17_52.value() as u128);
        // D s_17_56: size-of s_17_52
        let s_17_56: u16 = s_17_52.length();
        // D s_17_57: lsl s_17_53 s_17_56
        let s_17_57: u128 = s_17_53 << s_17_56;
        // D s_17_58: or s_17_57 s_17_55
        let s_17_58: u128 = ((s_17_57) | (s_17_55));
        // D s_17_59: add s_17_54 s_17_56
        let s_17_59: u16 = (s_17_54 + s_17_56);
        // D s_17_60: create-bits s_17_58 s_17_59
        let s_17_60: Bits = Bits::new(s_17_58, s_17_59);
        // D s_17_61: cast reint s_17_60 -> u8
        let s_17_61: u8 = (s_17_60.value() as u8);
        // D s_17_62: cast zx s_17_30 -> bv
        let s_17_62: Bits = Bits::new(s_17_30 as u128, 3u16);
        // D s_17_63: cast zx s_17_61 -> bv
        let s_17_63: Bits = Bits::new(s_17_61 as u128, 3u16);
        // D s_17_64: and s_17_62 s_17_63
        let s_17_64: Bits = ((s_17_62) & (s_17_63));
        // D s_17_65: cast reint s_17_64 -> u8
        let s_17_65: u8 = (s_17_64.value() as u8);
        // D s_17_66: write-var pending <= s_17_65
        fn_state.pending = s_17_65;
        // N s_17_67: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call HCR_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_18_0);
        // S s_18_2: call _get_HCR_Type_TGE(s_18_1)
        let s_18_2: bool = u_get_HCR_Type_TGE(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #0u : u8
        let s_18_4: bool = false;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // S s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // D s_18_7: write-var gs#327537 <= s_18_6
        fn_state.gs_327537 = s_18_6;
        // N s_18_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call EL2Enabled(s_19_0)
        let s_19_1: bool = EL2Enabled(state, tracer, s_19_0);
        // D s_19_2: write-var gs#327536 <= s_19_1
        fn_state.gs_327536 = s_19_1;
        // N s_19_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#327535 <= s_20_0
        fn_state.gs_327535 = s_20_0;
        // N s_20_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_21_0: const #16968u : u32
        let s_21_0: u32 = 16968;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: bool = {
            let value = state.read_register::<bool>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #16979u : u32
        let s_21_2: u32 = 16979;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: bool = {
            let value = state.read_register::<bool>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // C s_21_4: const #16977u : u32
        let s_21_4: u32 = 16977;
        // D s_21_5: read-reg s_21_4:u8
        let s_21_5: bool = {
            let value = state.read_register::<bool>(s_21_4 as isize);
            tracer.read_register(s_21_4 as isize, value);
            value
        };
        // D s_21_6: cast zx s_21_3 -> bv
        let s_21_6: Bits = Bits::new(s_21_3 as u128, 1u16);
        // D s_21_7: cast zx s_21_5 -> bv
        let s_21_7: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_8: cast reint s_21_6 -> u128
        let s_21_8: u128 = (s_21_6.value() as u128);
        // D s_21_9: size-of s_21_6
        let s_21_9: u16 = s_21_6.length();
        // D s_21_10: cast reint s_21_7 -> u128
        let s_21_10: u128 = (s_21_7.value() as u128);
        // D s_21_11: size-of s_21_7
        let s_21_11: u16 = s_21_7.length();
        // D s_21_12: lsl s_21_8 s_21_11
        let s_21_12: u128 = s_21_8 << s_21_11;
        // D s_21_13: or s_21_12 s_21_10
        let s_21_13: u128 = ((s_21_12) | (s_21_10));
        // D s_21_14: add s_21_9 s_21_11
        let s_21_14: u16 = (s_21_9 + s_21_11);
        // D s_21_15: create-bits s_21_13 s_21_14
        let s_21_15: Bits = Bits::new(s_21_13, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u8
        let s_21_16: u8 = (s_21_15.value() as u8);
        // D s_21_17: cast zx s_21_1 -> bv
        let s_21_17: Bits = Bits::new(s_21_1 as u128, 1u16);
        // D s_21_18: cast zx s_21_16 -> bv
        let s_21_18: Bits = Bits::new(s_21_16 as u128, 2u16);
        // D s_21_19: cast reint s_21_17 -> u128
        let s_21_19: u128 = (s_21_17.value() as u128);
        // D s_21_20: size-of s_21_17
        let s_21_20: u16 = s_21_17.length();
        // D s_21_21: cast reint s_21_18 -> u128
        let s_21_21: u128 = (s_21_18.value() as u128);
        // D s_21_22: size-of s_21_18
        let s_21_22: u16 = s_21_18.length();
        // D s_21_23: lsl s_21_19 s_21_22
        let s_21_23: u128 = s_21_19 << s_21_22;
        // D s_21_24: or s_21_23 s_21_21
        let s_21_24: u128 = ((s_21_23) | (s_21_21));
        // D s_21_25: add s_21_20 s_21_22
        let s_21_25: u16 = (s_21_20 + s_21_22);
        // D s_21_26: create-bits s_21_24 s_21_25
        let s_21_26: Bits = Bits::new(s_21_24, s_21_25);
        // D s_21_27: cast reint s_21_26 -> u8
        let s_21_27: u8 = (s_21_26.value() as u8);
        // D s_21_28: call AArch64_PendingUnmaskedVirtualInterrupts(s_21_27)
        let s_21_28: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedVirtualInterrupts(
            state,
            tracer,
            s_21_27,
        );
        // D s_21_29: write-var return_value <= s_21_28
        fn_state.return_value = s_21_28;
        // N s_21_30: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // D s_22_4: write-var gs#327532 <= s_22_3
        fn_state.gs_327532 = s_22_3;
        // N s_22_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#327533 <= s_23_0
        fn_state.gs_327533 = s_23_0;
        // N s_23_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTyped8f896a024a4e2cb {
        // C s_24_0: const #432u : u32
        let s_24_0: u32 = 432;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call ELUsingAArch32(s_24_1)
        let s_24_2: bool = ELUsingAArch32(state, tracer, s_24_1);
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // D s_24_4: write-var gs#327531 <= s_24_3
        fn_state.gs_327531 = s_24_3;
        // N s_24_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
