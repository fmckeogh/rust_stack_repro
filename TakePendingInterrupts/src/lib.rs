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
use AArch64_TakeVirtualIRQException::*;
use AArch32_TakeVirtualIRQException::*;
use AArch32_TakeVirtualFIQException::*;
use AArch64_TakePhysicalSErrorException::*;
use AArch32_TakeVirtualSErrorException::*;
use AArch64_TakePhysicalIRQException::*;
use AArch64_TakePhysicalFIQException::*;
use Zeros::*;
use AArch64_PendingUnmaskedVirtualInterrupts::*;
use AArch32_PendingUnmaskedPhysicalInterrupts::*;
use HaveTME::*;
use AArch32_PendingUnmaskedVirtualInterrupts::*;
use AArch64_TakeVirtualSErrorException::*;
use AArch64_PendingUnmaskedPhysicalInterrupts::*;
use AArch32_TakePhysicalIRQException::*;
use AArch32_TakePhysicalFIQException::*;
use AArch32_TakePhysicalSErrorException::*;
use FailTransaction__1::*;
use UsingAArch32::*;
use AArch64_TakeVirtualFIQException::*;
use common::*;
pub fn TakePendingInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    interrupt_req: ProductTypefe062afb059b3bbc,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_367875: ProductTyped8f896a024a4e2cb,
        gs_327562: bool,
        gs_327582: bool,
        gs_327584: bool,
        ga_367837: ProductTyped8f896a024a4e2cb,
        ga_367843: ProductTyped8f896a024a4e2cb,
        SE: bool,
        gs_327581: bool,
        gs_327603: bool,
        gs_327556: bool,
        gs_327580: bool,
        ga_367869: ProductTyped8f896a024a4e2cb,
        gs_327606: bool,
        gs_327604: bool,
        gs_327558: bool,
        gs_327602: bool,
        ga_367813: ProductTyped8f896a024a4e2cb,
        gs_327557: bool,
        vSE: bool,
        gs_327555: bool,
        FIQ: bool,
        vIRQ: bool,
        gs_327583: bool,
        gs_327605: bool,
        vFIQ: bool,
        AA: bool,
        IRQ: bool,
        ga_367812: ProductTyped8f896a024a4e2cb,
        gs_327559: bool,
        interrupt_taken: bool,
        vAA: bool,
        interrupt_req: ProductTypefe062afb059b3bbc,
    }
    let fn_state = FunctionState {
        interrupt_req,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b90 b1
        if s_0_1 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #16968u : u32
        let s_1_0: u32 = 16968;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #16979u : u32
        let s_1_2: u32 = 16979;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: bool = {
            let value = state.read_register::<bool>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // C s_1_4: const #16977u : u32
        let s_1_4: u32 = 16977;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: bool = {
            let value = state.read_register::<bool>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: cast zx s_1_3 -> bv
        let s_1_6: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 1u16);
        // D s_1_8: cast reint s_1_6 -> u128
        let s_1_8: u128 = (s_1_6.value() as u128);
        // D s_1_9: size-of s_1_6
        let s_1_9: u16 = s_1_6.length();
        // D s_1_10: cast reint s_1_7 -> u128
        let s_1_10: u128 = (s_1_7.value() as u128);
        // D s_1_11: size-of s_1_7
        let s_1_11: u16 = s_1_7.length();
        // D s_1_12: lsl s_1_8 s_1_11
        let s_1_12: u128 = s_1_8 << s_1_11;
        // D s_1_13: or s_1_12 s_1_10
        let s_1_13: u128 = ((s_1_12) | (s_1_10));
        // D s_1_14: add s_1_9 s_1_11
        let s_1_14: u16 = (s_1_9 + s_1_11);
        // D s_1_15: create-bits s_1_13 s_1_14
        let s_1_15: Bits = Bits::new(s_1_13, s_1_14);
        // D s_1_16: cast reint s_1_15 -> u8
        let s_1_16: u8 = (s_1_15.value() as u8);
        // D s_1_17: cast zx s_1_1 -> bv
        let s_1_17: Bits = Bits::new(s_1_1 as u128, 1u16);
        // D s_1_18: cast zx s_1_16 -> bv
        let s_1_18: Bits = Bits::new(s_1_16 as u128, 2u16);
        // D s_1_19: cast reint s_1_17 -> u128
        let s_1_19: u128 = (s_1_17.value() as u128);
        // D s_1_20: size-of s_1_17
        let s_1_20: u16 = s_1_17.length();
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: lsl s_1_19 s_1_22
        let s_1_23: u128 = s_1_19 << s_1_22;
        // D s_1_24: or s_1_23 s_1_21
        let s_1_24: u128 = ((s_1_23) | (s_1_21));
        // D s_1_25: add s_1_20 s_1_22
        let s_1_25: u16 = (s_1_20 + s_1_22);
        // D s_1_26: create-bits s_1_24 s_1_25
        let s_1_26: Bits = Bits::new(s_1_24, s_1_25);
        // D s_1_27: cast reint s_1_26 -> u8
        let s_1_27: u8 = (s_1_26.value() as u8);
        // D s_1_28: call AArch64_PendingUnmaskedVirtualInterrupts(s_1_27)
        let s_1_28: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedVirtualInterrupts(
            state,
            tracer,
            s_1_27,
        );
        // D s_1_29: write-var ga#367837 <= s_1_28
        fn_state.ga_367837 = s_1_28;
        // D s_1_30: read-var ga#367837.0:struct
        let s_1_30: bool = fn_state.ga_367837._0;
        // D s_1_31: read-var ga#367837.1:struct
        let s_1_31: bool = fn_state.ga_367837._1;
        // D s_1_32: read-var ga#367837.2:struct
        let s_1_32: bool = fn_state.ga_367837._2;
        // D s_1_33: write-var vSE <= s_1_30
        fn_state.vSE = s_1_30;
        // D s_1_34: write-var vIRQ <= s_1_31
        fn_state.vIRQ = s_1_31;
        // D s_1_35: write-var vFIQ <= s_1_32
        fn_state.vFIQ = s_1_32;
        // C s_1_36: const #16968u : u32
        let s_1_36: u32 = 16968;
        // D s_1_37: read-reg s_1_36:u8
        let s_1_37: bool = {
            let value = state.read_register::<bool>(s_1_36 as isize);
            tracer.read_register(s_1_36 as isize, value);
            value
        };
        // C s_1_38: const #16979u : u32
        let s_1_38: u32 = 16979;
        // D s_1_39: read-reg s_1_38:u8
        let s_1_39: bool = {
            let value = state.read_register::<bool>(s_1_38 as isize);
            tracer.read_register(s_1_38 as isize, value);
            value
        };
        // C s_1_40: const #16977u : u32
        let s_1_40: u32 = 16977;
        // D s_1_41: read-reg s_1_40:u8
        let s_1_41: bool = {
            let value = state.read_register::<bool>(s_1_40 as isize);
            tracer.read_register(s_1_40 as isize, value);
            value
        };
        // D s_1_42: cast zx s_1_39 -> bv
        let s_1_42: Bits = Bits::new(s_1_39 as u128, 1u16);
        // D s_1_43: cast zx s_1_41 -> bv
        let s_1_43: Bits = Bits::new(s_1_41 as u128, 1u16);
        // D s_1_44: cast reint s_1_42 -> u128
        let s_1_44: u128 = (s_1_42.value() as u128);
        // D s_1_45: size-of s_1_42
        let s_1_45: u16 = s_1_42.length();
        // D s_1_46: cast reint s_1_43 -> u128
        let s_1_46: u128 = (s_1_43.value() as u128);
        // D s_1_47: size-of s_1_43
        let s_1_47: u16 = s_1_43.length();
        // D s_1_48: lsl s_1_44 s_1_47
        let s_1_48: u128 = s_1_44 << s_1_47;
        // D s_1_49: or s_1_48 s_1_46
        let s_1_49: u128 = ((s_1_48) | (s_1_46));
        // D s_1_50: add s_1_45 s_1_47
        let s_1_50: u16 = (s_1_45 + s_1_47);
        // D s_1_51: create-bits s_1_49 s_1_50
        let s_1_51: Bits = Bits::new(s_1_49, s_1_50);
        // D s_1_52: cast reint s_1_51 -> u8
        let s_1_52: u8 = (s_1_51.value() as u8);
        // D s_1_53: cast zx s_1_37 -> bv
        let s_1_53: Bits = Bits::new(s_1_37 as u128, 1u16);
        // D s_1_54: cast zx s_1_52 -> bv
        let s_1_54: Bits = Bits::new(s_1_52 as u128, 2u16);
        // D s_1_55: cast reint s_1_53 -> u128
        let s_1_55: u128 = (s_1_53.value() as u128);
        // D s_1_56: size-of s_1_53
        let s_1_56: u16 = s_1_53.length();
        // D s_1_57: cast reint s_1_54 -> u128
        let s_1_57: u128 = (s_1_54.value() as u128);
        // D s_1_58: size-of s_1_54
        let s_1_58: u16 = s_1_54.length();
        // D s_1_59: lsl s_1_55 s_1_58
        let s_1_59: u128 = s_1_55 << s_1_58;
        // D s_1_60: or s_1_59 s_1_57
        let s_1_60: u128 = ((s_1_59) | (s_1_57));
        // D s_1_61: add s_1_56 s_1_58
        let s_1_61: u16 = (s_1_56 + s_1_58);
        // D s_1_62: create-bits s_1_60 s_1_61
        let s_1_62: Bits = Bits::new(s_1_60, s_1_61);
        // D s_1_63: cast reint s_1_62 -> u8
        let s_1_63: u8 = (s_1_62.value() as u8);
        // D s_1_64: call AArch64_PendingUnmaskedPhysicalInterrupts(s_1_63)
        let s_1_64: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_1_63,
        );
        // D s_1_65: write-var ga#367843 <= s_1_64
        fn_state.ga_367843 = s_1_64;
        // D s_1_66: read-var ga#367843.0:struct
        let s_1_66: bool = fn_state.ga_367843._0;
        // D s_1_67: read-var ga#367843.1:struct
        let s_1_67: bool = fn_state.ga_367843._1;
        // D s_1_68: read-var ga#367843.2:struct
        let s_1_68: bool = fn_state.ga_367843._2;
        // D s_1_69: write-var SE <= s_1_66
        fn_state.SE = s_1_66;
        // D s_1_70: write-var IRQ <= s_1_67
        fn_state.IRQ = s_1_67;
        // D s_1_71: write-var FIQ <= s_1_68
        fn_state.FIQ = s_1_68;
        // D s_1_72: read-var interrupt_req.3:struct
        let s_1_72: bool = fn_state.interrupt_req._3;
        // D s_1_73: not s_1_72
        let s_1_73: bool = !s_1_72;
        // N s_1_74: branch s_1_73 b89 b2
        if s_1_73 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var interrupt_req.6:struct
        let s_3_0: bool = fn_state.interrupt_req._6;
        // D s_3_1: not s_3_0
        let s_3_1: bool = !s_3_0;
        // N s_3_2: branch s_3_1 b88 b4
        if s_3_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var interrupt_req.2:struct
        let s_5_0: bool = fn_state.interrupt_req._2;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // N s_5_2: branch s_5_1 b87 b6
        if s_5_1 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var interrupt_req.5:struct
        let s_7_0: bool = fn_state.interrupt_req._5;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b86 b8
        if s_7_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var interrupt_req.1:struct
        let s_9_0: bool = fn_state.interrupt_req._1;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b85 b10
        if s_9_1 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var interrupt_req.4:struct
        let s_11_0: bool = fn_state.interrupt_req._4;
        // D s_11_1: not s_11_0
        let s_11_1: bool = !s_11_0;
        // N s_11_2: branch s_11_1 b84 b12
        if s_11_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var SE:u8
        let s_13_0: bool = fn_state.SE;
        // N s_13_1: branch s_13_0 b83 b14
        if s_13_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var FIQ:u8
        let s_14_0: bool = fn_state.FIQ;
        // D s_14_1: write-var gs#327555 <= s_14_0
        fn_state.gs_327555 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#327555:u8
        let s_15_0: bool = fn_state.gs_327555;
        // N s_15_1: branch s_15_0 b82 b16
        if s_15_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var IRQ:u8
        let s_16_0: bool = fn_state.IRQ;
        // D s_16_1: write-var gs#327556 <= s_16_0
        fn_state.gs_327556 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var gs#327556:u8
        let s_17_0: bool = fn_state.gs_327556;
        // N s_17_1: branch s_17_0 b81 b18
        if s_17_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var vSE:u8
        let s_18_0: bool = fn_state.vSE;
        // D s_18_1: write-var gs#327557 <= s_18_0
        fn_state.gs_327557 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var gs#327557:u8
        let s_19_0: bool = fn_state.gs_327557;
        // N s_19_1: branch s_19_0 b80 b20
        if s_19_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var vFIQ:u8
        let s_20_0: bool = fn_state.vFIQ;
        // D s_20_1: write-var gs#327558 <= s_20_0
        fn_state.gs_327558 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var gs#327558:u8
        let s_21_0: bool = fn_state.gs_327558;
        // N s_21_1: branch s_21_0 b79 b22
        if s_21_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var vIRQ:u8
        let s_22_0: bool = fn_state.vIRQ;
        // D s_22_1: write-var gs#327559 <= s_22_0
        fn_state.gs_327559 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#327559:u8
        let s_23_0: bool = fn_state.gs_327559;
        // N s_23_1: branch s_23_0 b39 b24
        if s_23_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var interrupt_taken <= s_24_0
        fn_state.interrupt_taken = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var vFIQ:u8
        let s_25_0: bool = fn_state.vFIQ;
        // N s_25_1: branch s_25_0 b38 b26
        if s_25_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var vIRQ:u8
        let s_26_0: bool = fn_state.vIRQ;
        // N s_26_1: branch s_26_0 b37 b27
        if s_26_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var vSE:u8
        let s_27_0: bool = fn_state.vSE;
        // N s_27_1: branch s_27_0 b36 b28
        if s_27_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var FIQ:u8
        let s_28_0: bool = fn_state.FIQ;
        // N s_28_1: branch s_28_0 b35 b29
        if s_28_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var IRQ:u8
        let s_29_0: bool = fn_state.IRQ;
        // N s_29_1: branch s_29_0 b34 b30
        if s_29_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var SE:u8
        let s_30_0: bool = fn_state.SE;
        // N s_30_1: branch s_30_0 b33 b31
        if s_30_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var interrupt_taken:u8
        let s_32_0: bool = fn_state.interrupt_taken;
        // N s_32_1: return s_32_0
        return s_32_0;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var interrupt_req.0:struct
        let s_33_0: bool = fn_state.interrupt_req._0;
        // D s_33_1: call AArch64_TakePhysicalSErrorException(s_33_0)
        let s_33_1: () = AArch64_TakePhysicalSErrorException(state, tracer, s_33_0);
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call AArch64_TakePhysicalIRQException(s_34_0)
        let s_34_1: () = AArch64_TakePhysicalIRQException(state, tracer, s_34_0);
        // N s_34_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call AArch64_TakePhysicalFIQException(s_35_0)
        let s_35_1: () = AArch64_TakePhysicalFIQException(state, tracer, s_35_0);
        // N s_35_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call AArch64_TakeVirtualSErrorException(s_36_0)
        let s_36_1: () = AArch64_TakeVirtualSErrorException(state, tracer, s_36_0);
        // N s_36_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call AArch64_TakeVirtualIRQException(s_37_0)
        let s_37_1: () = AArch64_TakeVirtualIRQException(state, tracer, s_37_0);
        // N s_37_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call AArch64_TakeVirtualFIQException(s_38_0)
        let s_38_1: () = AArch64_TakeVirtualFIQException(state, tracer, s_38_0);
        // N s_38_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HaveTME(s_39_0)
        let s_39_1: bool = HaveTME(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b78 b40
        if s_39_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#327562 <= s_40_0
        fn_state.gs_327562 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_41_0: read-var gs#327562:u8
        let s_41_0: bool = fn_state.gs_327562;
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
    ) -> bool {
        // N s_42_0: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var interrupt_taken <= s_43_0
        fn_state.interrupt_taken = s_43_0;
        // N s_43_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #91136u : u32
        let s_44_0: u32 = 91136;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: bool = {
            let value = state.read_register::<bool>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // C s_44_2: const #91195u : u32
        let s_44_2: u32 = 91195;
        // D s_44_3: read-reg s_44_2:u8
        let s_44_3: bool = {
            let value = state.read_register::<bool>(s_44_2 as isize);
            tracer.read_register(s_44_2 as isize, value);
            value
        };
        // C s_44_4: const #91138u : u32
        let s_44_4: u32 = 91138;
        // D s_44_5: read-reg s_44_4:u8
        let s_44_5: bool = {
            let value = state.read_register::<bool>(s_44_4 as isize);
            tracer.read_register(s_44_4 as isize, value);
            value
        };
        // D s_44_6: cast zx s_44_3 -> bv
        let s_44_6: Bits = Bits::new(s_44_3 as u128, 1u16);
        // D s_44_7: cast zx s_44_5 -> bv
        let s_44_7: Bits = Bits::new(s_44_5 as u128, 1u16);
        // D s_44_8: cast reint s_44_6 -> u128
        let s_44_8: u128 = (s_44_6.value() as u128);
        // D s_44_9: size-of s_44_6
        let s_44_9: u16 = s_44_6.length();
        // D s_44_10: cast reint s_44_7 -> u128
        let s_44_10: u128 = (s_44_7.value() as u128);
        // D s_44_11: size-of s_44_7
        let s_44_11: u16 = s_44_7.length();
        // D s_44_12: lsl s_44_8 s_44_11
        let s_44_12: u128 = s_44_8 << s_44_11;
        // D s_44_13: or s_44_12 s_44_10
        let s_44_13: u128 = ((s_44_12) | (s_44_10));
        // D s_44_14: add s_44_9 s_44_11
        let s_44_14: u16 = (s_44_9 + s_44_11);
        // D s_44_15: create-bits s_44_13 s_44_14
        let s_44_15: Bits = Bits::new(s_44_13, s_44_14);
        // D s_44_16: cast reint s_44_15 -> u8
        let s_44_16: u8 = (s_44_15.value() as u8);
        // D s_44_17: cast zx s_44_1 -> bv
        let s_44_17: Bits = Bits::new(s_44_1 as u128, 1u16);
        // D s_44_18: cast zx s_44_16 -> bv
        let s_44_18: Bits = Bits::new(s_44_16 as u128, 2u16);
        // D s_44_19: cast reint s_44_17 -> u128
        let s_44_19: u128 = (s_44_17.value() as u128);
        // D s_44_20: size-of s_44_17
        let s_44_20: u16 = s_44_17.length();
        // D s_44_21: cast reint s_44_18 -> u128
        let s_44_21: u128 = (s_44_18.value() as u128);
        // D s_44_22: size-of s_44_18
        let s_44_22: u16 = s_44_18.length();
        // D s_44_23: lsl s_44_19 s_44_22
        let s_44_23: u128 = s_44_19 << s_44_22;
        // D s_44_24: or s_44_23 s_44_21
        let s_44_24: u128 = ((s_44_23) | (s_44_21));
        // D s_44_25: add s_44_20 s_44_22
        let s_44_25: u16 = (s_44_20 + s_44_22);
        // D s_44_26: create-bits s_44_24 s_44_25
        let s_44_26: Bits = Bits::new(s_44_24, s_44_25);
        // D s_44_27: cast reint s_44_26 -> u8
        let s_44_27: u8 = (s_44_26.value() as u8);
        // D s_44_28: call AArch64_PendingUnmaskedVirtualInterrupts(s_44_27)
        let s_44_28: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedVirtualInterrupts(
            state,
            tracer,
            s_44_27,
        );
        // D s_44_29: write-var ga#367869 <= s_44_28
        fn_state.ga_367869 = s_44_28;
        // D s_44_30: read-var ga#367869.0:struct
        let s_44_30: bool = fn_state.ga_367869._0;
        // D s_44_31: read-var ga#367869.1:struct
        let s_44_31: bool = fn_state.ga_367869._1;
        // D s_44_32: read-var ga#367869.2:struct
        let s_44_32: bool = fn_state.ga_367869._2;
        // D s_44_33: write-var vSE <= s_44_30
        fn_state.vSE = s_44_30;
        // D s_44_34: write-var vIRQ <= s_44_31
        fn_state.vIRQ = s_44_31;
        // D s_44_35: write-var vFIQ <= s_44_32
        fn_state.vFIQ = s_44_32;
        // C s_44_36: const #91136u : u32
        let s_44_36: u32 = 91136;
        // D s_44_37: read-reg s_44_36:u8
        let s_44_37: bool = {
            let value = state.read_register::<bool>(s_44_36 as isize);
            tracer.read_register(s_44_36 as isize, value);
            value
        };
        // C s_44_38: const #91195u : u32
        let s_44_38: u32 = 91195;
        // D s_44_39: read-reg s_44_38:u8
        let s_44_39: bool = {
            let value = state.read_register::<bool>(s_44_38 as isize);
            tracer.read_register(s_44_38 as isize, value);
            value
        };
        // C s_44_40: const #91138u : u32
        let s_44_40: u32 = 91138;
        // D s_44_41: read-reg s_44_40:u8
        let s_44_41: bool = {
            let value = state.read_register::<bool>(s_44_40 as isize);
            tracer.read_register(s_44_40 as isize, value);
            value
        };
        // D s_44_42: cast zx s_44_39 -> bv
        let s_44_42: Bits = Bits::new(s_44_39 as u128, 1u16);
        // D s_44_43: cast zx s_44_41 -> bv
        let s_44_43: Bits = Bits::new(s_44_41 as u128, 1u16);
        // D s_44_44: cast reint s_44_42 -> u128
        let s_44_44: u128 = (s_44_42.value() as u128);
        // D s_44_45: size-of s_44_42
        let s_44_45: u16 = s_44_42.length();
        // D s_44_46: cast reint s_44_43 -> u128
        let s_44_46: u128 = (s_44_43.value() as u128);
        // D s_44_47: size-of s_44_43
        let s_44_47: u16 = s_44_43.length();
        // D s_44_48: lsl s_44_44 s_44_47
        let s_44_48: u128 = s_44_44 << s_44_47;
        // D s_44_49: or s_44_48 s_44_46
        let s_44_49: u128 = ((s_44_48) | (s_44_46));
        // D s_44_50: add s_44_45 s_44_47
        let s_44_50: u16 = (s_44_45 + s_44_47);
        // D s_44_51: create-bits s_44_49 s_44_50
        let s_44_51: Bits = Bits::new(s_44_49, s_44_50);
        // D s_44_52: cast reint s_44_51 -> u8
        let s_44_52: u8 = (s_44_51.value() as u8);
        // D s_44_53: cast zx s_44_37 -> bv
        let s_44_53: Bits = Bits::new(s_44_37 as u128, 1u16);
        // D s_44_54: cast zx s_44_52 -> bv
        let s_44_54: Bits = Bits::new(s_44_52 as u128, 2u16);
        // D s_44_55: cast reint s_44_53 -> u128
        let s_44_55: u128 = (s_44_53.value() as u128);
        // D s_44_56: size-of s_44_53
        let s_44_56: u16 = s_44_53.length();
        // D s_44_57: cast reint s_44_54 -> u128
        let s_44_57: u128 = (s_44_54.value() as u128);
        // D s_44_58: size-of s_44_54
        let s_44_58: u16 = s_44_54.length();
        // D s_44_59: lsl s_44_55 s_44_58
        let s_44_59: u128 = s_44_55 << s_44_58;
        // D s_44_60: or s_44_59 s_44_57
        let s_44_60: u128 = ((s_44_59) | (s_44_57));
        // D s_44_61: add s_44_56 s_44_58
        let s_44_61: u16 = (s_44_56 + s_44_58);
        // D s_44_62: create-bits s_44_60 s_44_61
        let s_44_62: Bits = Bits::new(s_44_60, s_44_61);
        // D s_44_63: cast reint s_44_62 -> u8
        let s_44_63: u8 = (s_44_62.value() as u8);
        // D s_44_64: call AArch64_PendingUnmaskedPhysicalInterrupts(s_44_63)
        let s_44_64: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_44_63,
        );
        // D s_44_65: write-var ga#367875 <= s_44_64
        fn_state.ga_367875 = s_44_64;
        // D s_44_66: read-var ga#367875.0:struct
        let s_44_66: bool = fn_state.ga_367875._0;
        // D s_44_67: read-var ga#367875.1:struct
        let s_44_67: bool = fn_state.ga_367875._1;
        // D s_44_68: read-var ga#367875.2:struct
        let s_44_68: bool = fn_state.ga_367875._2;
        // D s_44_69: write-var SE <= s_44_66
        fn_state.SE = s_44_66;
        // D s_44_70: write-var IRQ <= s_44_67
        fn_state.IRQ = s_44_67;
        // D s_44_71: write-var FIQ <= s_44_68
        fn_state.FIQ = s_44_68;
        // D s_44_72: read-var interrupt_req.3:struct
        let s_44_72: bool = fn_state.interrupt_req._3;
        // D s_44_73: not s_44_72
        let s_44_73: bool = !s_44_72;
        // N s_44_74: branch s_44_73 b77 b45
        if s_44_73 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var interrupt_req.6:struct
        let s_46_0: bool = fn_state.interrupt_req._6;
        // D s_46_1: not s_46_0
        let s_46_1: bool = !s_46_0;
        // N s_46_2: branch s_46_1 b76 b47
        if s_46_1 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var interrupt_req.2:struct
        let s_48_0: bool = fn_state.interrupt_req._2;
        // D s_48_1: not s_48_0
        let s_48_1: bool = !s_48_0;
        // N s_48_2: branch s_48_1 b75 b49
        if s_48_1 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_49_0: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var interrupt_req.5:struct
        let s_50_0: bool = fn_state.interrupt_req._5;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b74 b51
        if s_50_1 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_52_0: read-var interrupt_req.1:struct
        let s_52_0: bool = fn_state.interrupt_req._1;
        // D s_52_1: not s_52_0
        let s_52_1: bool = !s_52_0;
        // N s_52_2: branch s_52_1 b73 b53
        if s_52_1 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_53_0: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var interrupt_req.4:struct
        let s_54_0: bool = fn_state.interrupt_req._4;
        // D s_54_1: not s_54_0
        let s_54_1: bool = !s_54_0;
        // N s_54_2: branch s_54_1 b72 b55
        if s_54_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var SE:u8
        let s_56_0: bool = fn_state.SE;
        // N s_56_1: branch s_56_0 b71 b57
        if s_56_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_57_0: read-var FIQ:u8
        let s_57_0: bool = fn_state.FIQ;
        // D s_57_1: write-var gs#327580 <= s_57_0
        fn_state.gs_327580 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var gs#327580:u8
        let s_58_0: bool = fn_state.gs_327580;
        // N s_58_1: branch s_58_0 b70 b59
        if s_58_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_59_0: read-var IRQ:u8
        let s_59_0: bool = fn_state.IRQ;
        // D s_59_1: write-var gs#327581 <= s_59_0
        fn_state.gs_327581 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_60_0: read-var gs#327581:u8
        let s_60_0: bool = fn_state.gs_327581;
        // N s_60_1: branch s_60_0 b69 b61
        if s_60_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_61_0: read-var vSE:u8
        let s_61_0: bool = fn_state.vSE;
        // D s_61_1: write-var gs#327582 <= s_61_0
        fn_state.gs_327582 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_62_0: read-var gs#327582:u8
        let s_62_0: bool = fn_state.gs_327582;
        // N s_62_1: branch s_62_0 b68 b63
        if s_62_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var vFIQ:u8
        let s_63_0: bool = fn_state.vFIQ;
        // D s_63_1: write-var gs#327583 <= s_63_0
        fn_state.gs_327583 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var gs#327583:u8
        let s_64_0: bool = fn_state.gs_327583;
        // N s_64_1: branch s_64_0 b67 b65
        if s_64_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_65_0: read-var vIRQ:u8
        let s_65_0: bool = fn_state.vIRQ;
        // D s_65_1: write-var gs#327584 <= s_65_0
        fn_state.gs_327584 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_66_0: read-var gs#327584:u8
        let s_66_0: bool = fn_state.gs_327584;
        // D s_66_1: not s_66_0
        let s_66_1: bool = !s_66_0;
        // C s_66_2: const #15s : i
        let s_66_2: i128 = 15;
        // S s_66_3: call Zeros(s_66_2)
        let s_66_3: Bits = Zeros(state, tracer, s_66_2);
        // S s_66_4: cast reint s_66_3 -> u15
        let s_66_4: u16 = (s_66_3.value() as u16);
        // C s_66_5: const #7u : u32
        let s_66_5: u32 = 7;
        // D s_66_6: call FailTransaction__1(s_66_5, s_66_0, s_66_1, s_66_4)
        let s_66_6: () = FailTransaction__1(
            state,
            tracer,
            s_66_5,
            s_66_0,
            s_66_1,
            s_66_4,
        );
        // N s_66_7: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#327584 <= s_67_0
        fn_state.gs_327584 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#327583 <= s_68_0
        fn_state.gs_327583 = s_68_0;
        // N s_68_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#327582 <= s_69_0
        fn_state.gs_327582 = s_69_0;
        // N s_69_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#327581 <= s_70_0
        fn_state.gs_327581 = s_70_0;
        // N s_70_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#327580 <= s_71_0
        fn_state.gs_327580 = s_71_0;
        // N s_71_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var vFIQ <= s_72_0
        fn_state.vFIQ = s_72_0;
        // N s_72_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var FIQ <= s_73_0
        fn_state.FIQ = s_73_0;
        // N s_73_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var vIRQ <= s_74_0
        fn_state.vIRQ = s_74_0;
        // N s_74_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var IRQ <= s_75_0
        fn_state.IRQ = s_75_0;
        // N s_75_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var vSE <= s_76_0
        fn_state.vSE = s_76_0;
        // N s_76_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var SE <= s_77_0
        fn_state.SE = s_77_0;
        // N s_77_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_78_0: const #100180u : u32
        let s_78_0: u32 = 100180;
        // D s_78_1: read-reg s_78_0:i
        let s_78_1: i128 = {
            let value = state.read_register::<i128>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // C s_78_2: const #0s : i
        let s_78_2: i128 = 0;
        // D s_78_3: cmp-gt s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) > (s_78_2));
        // D s_78_4: write-var gs#327562 <= s_78_3
        fn_state.gs_327562 = s_78_3;
        // N s_78_5: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#327559 <= s_79_0
        fn_state.gs_327559 = s_79_0;
        // N s_79_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#327558 <= s_80_0
        fn_state.gs_327558 = s_80_0;
        // N s_80_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#327557 <= s_81_0
        fn_state.gs_327557 = s_81_0;
        // N s_81_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#327556 <= s_82_0
        fn_state.gs_327556 = s_82_0;
        // N s_82_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#327555 <= s_83_0
        fn_state.gs_327555 = s_83_0;
        // N s_83_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var vFIQ <= s_84_0
        fn_state.vFIQ = s_84_0;
        // N s_84_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var FIQ <= s_85_0
        fn_state.FIQ = s_85_0;
        // N s_85_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var vIRQ <= s_86_0
        fn_state.vIRQ = s_86_0;
        // N s_86_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var IRQ <= s_87_0
        fn_state.IRQ = s_87_0;
        // N s_87_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var vSE <= s_88_0
        fn_state.vSE = s_88_0;
        // N s_88_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var SE <= s_89_0
        fn_state.SE = s_89_0;
        // N s_89_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call AArch32_PendingUnmaskedVirtualInterrupts(s_90_0)
        let s_90_1: ProductTyped8f896a024a4e2cb = AArch32_PendingUnmaskedVirtualInterrupts(
            state,
            tracer,
            s_90_0,
        );
        // D s_90_2: write-var ga#367812 <= s_90_1
        fn_state.ga_367812 = s_90_1;
        // D s_90_3: read-var ga#367812.0:struct
        let s_90_3: bool = fn_state.ga_367812._0;
        // D s_90_4: read-var ga#367812.1:struct
        let s_90_4: bool = fn_state.ga_367812._1;
        // D s_90_5: read-var ga#367812.2:struct
        let s_90_5: bool = fn_state.ga_367812._2;
        // D s_90_6: write-var vAA <= s_90_3
        fn_state.vAA = s_90_3;
        // D s_90_7: write-var vIRQ <= s_90_4
        fn_state.vIRQ = s_90_4;
        // D s_90_8: write-var vFIQ <= s_90_5
        fn_state.vFIQ = s_90_5;
        // C s_90_9: const #() : ()
        let s_90_9: () = ();
        // S s_90_10: call AArch32_PendingUnmaskedPhysicalInterrupts(s_90_9)
        let s_90_10: ProductTyped8f896a024a4e2cb = AArch32_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_90_9,
        );
        // D s_90_11: write-var ga#367813 <= s_90_10
        fn_state.ga_367813 = s_90_10;
        // D s_90_12: read-var ga#367813.0:struct
        let s_90_12: bool = fn_state.ga_367813._0;
        // D s_90_13: read-var ga#367813.1:struct
        let s_90_13: bool = fn_state.ga_367813._1;
        // D s_90_14: read-var ga#367813.2:struct
        let s_90_14: bool = fn_state.ga_367813._2;
        // D s_90_15: write-var AA <= s_90_12
        fn_state.AA = s_90_12;
        // D s_90_16: write-var IRQ <= s_90_13
        fn_state.IRQ = s_90_13;
        // D s_90_17: write-var FIQ <= s_90_14
        fn_state.FIQ = s_90_14;
        // D s_90_18: read-var interrupt_req.3:struct
        let s_90_18: bool = fn_state.interrupt_req._3;
        // D s_90_19: not s_90_18
        let s_90_19: bool = !s_90_18;
        // N s_90_20: branch s_90_19 b138 b91
        if s_90_19 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_91_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_92_0: read-var interrupt_req.6:struct
        let s_92_0: bool = fn_state.interrupt_req._6;
        // D s_92_1: not s_92_0
        let s_92_1: bool = !s_92_0;
        // N s_92_2: branch s_92_1 b137 b93
        if s_92_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_93_0: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_94_0: read-var interrupt_req.2:struct
        let s_94_0: bool = fn_state.interrupt_req._2;
        // D s_94_1: not s_94_0
        let s_94_1: bool = !s_94_0;
        // N s_94_2: branch s_94_1 b136 b95
        if s_94_1 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_95_0: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_96_0: read-var interrupt_req.5:struct
        let s_96_0: bool = fn_state.interrupt_req._5;
        // D s_96_1: not s_96_0
        let s_96_1: bool = !s_96_0;
        // N s_96_2: branch s_96_1 b135 b97
        if s_96_1 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_97_0: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_98_0: read-var interrupt_req.1:struct
        let s_98_0: bool = fn_state.interrupt_req._1;
        // D s_98_1: not s_98_0
        let s_98_1: bool = !s_98_0;
        // N s_98_2: branch s_98_1 b134 b99
        if s_98_1 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_99_0: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_100_0: read-var interrupt_req.4:struct
        let s_100_0: bool = fn_state.interrupt_req._4;
        // D s_100_1: not s_100_0
        let s_100_1: bool = !s_100_0;
        // N s_100_2: branch s_100_1 b133 b101
        if s_100_1 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_101_0: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_102_0: read-var AA:u8
        let s_102_0: bool = fn_state.AA;
        // N s_102_1: branch s_102_0 b132 b103
        if s_102_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_103_0: read-var FIQ:u8
        let s_103_0: bool = fn_state.FIQ;
        // D s_103_1: write-var gs#327602 <= s_103_0
        fn_state.gs_327602 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_104_0: read-var gs#327602:u8
        let s_104_0: bool = fn_state.gs_327602;
        // N s_104_1: branch s_104_0 b131 b105
        if s_104_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_105_0: read-var IRQ:u8
        let s_105_0: bool = fn_state.IRQ;
        // D s_105_1: write-var gs#327603 <= s_105_0
        fn_state.gs_327603 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_106_0: read-var gs#327603:u8
        let s_106_0: bool = fn_state.gs_327603;
        // N s_106_1: branch s_106_0 b130 b107
        if s_106_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_107_0: read-var vAA:u8
        let s_107_0: bool = fn_state.vAA;
        // D s_107_1: write-var gs#327604 <= s_107_0
        fn_state.gs_327604 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_108_0: read-var gs#327604:u8
        let s_108_0: bool = fn_state.gs_327604;
        // N s_108_1: branch s_108_0 b129 b109
        if s_108_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_109_0: read-var vFIQ:u8
        let s_109_0: bool = fn_state.vFIQ;
        // D s_109_1: write-var gs#327605 <= s_109_0
        fn_state.gs_327605 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_110_0: read-var gs#327605:u8
        let s_110_0: bool = fn_state.gs_327605;
        // N s_110_1: branch s_110_0 b128 b111
        if s_110_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_111_0: read-var vIRQ:u8
        let s_111_0: bool = fn_state.vIRQ;
        // D s_111_1: write-var gs#327606 <= s_111_0
        fn_state.gs_327606 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_112_0: read-var gs#327606:u8
        let s_112_0: bool = fn_state.gs_327606;
        // N s_112_1: branch s_112_0 b127 b113
        if s_112_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var interrupt_taken <= s_113_0
        fn_state.interrupt_taken = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_114_0: read-var vFIQ:u8
        let s_114_0: bool = fn_state.vFIQ;
        // N s_114_1: branch s_114_0 b126 b115
        if s_114_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_115_0: read-var vIRQ:u8
        let s_115_0: bool = fn_state.vIRQ;
        // N s_115_1: branch s_115_0 b125 b116
        if s_115_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_116_0: read-var vAA:u8
        let s_116_0: bool = fn_state.vAA;
        // N s_116_1: branch s_116_0 b124 b117
        if s_116_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_117_0: read-var FIQ:u8
        let s_117_0: bool = fn_state.FIQ;
        // N s_117_1: branch s_117_0 b123 b118
        if s_117_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_118_0: read-var IRQ:u8
        let s_118_0: bool = fn_state.IRQ;
        // N s_118_1: branch s_118_0 b122 b119
        if s_118_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_119_0: read-var AA:u8
        let s_119_0: bool = fn_state.AA;
        // N s_119_1: branch s_119_0 b121 b120
        if s_119_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_120_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_121_0: read-var interrupt_req.0:struct
        let s_121_0: bool = fn_state.interrupt_req._0;
        // D s_121_1: call AArch32_TakePhysicalSErrorException(s_121_0)
        let s_121_1: () = AArch32_TakePhysicalSErrorException(state, tracer, s_121_0);
        // N s_121_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_122_0: const #() : ()
        let s_122_0: () = ();
        // S s_122_1: call AArch32_TakePhysicalIRQException(s_122_0)
        let s_122_1: () = AArch32_TakePhysicalIRQException(state, tracer, s_122_0);
        // N s_122_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call AArch32_TakePhysicalFIQException(s_123_0)
        let s_123_1: () = AArch32_TakePhysicalFIQException(state, tracer, s_123_0);
        // N s_123_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call AArch32_TakeVirtualSErrorException(s_124_0)
        let s_124_1: () = AArch32_TakeVirtualSErrorException(state, tracer, s_124_0);
        // N s_124_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_125_0: const #() : ()
        let s_125_0: () = ();
        // S s_125_1: call AArch32_TakeVirtualIRQException(s_125_0)
        let s_125_1: () = AArch32_TakeVirtualIRQException(state, tracer, s_125_0);
        // N s_125_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call AArch32_TakeVirtualFIQException(s_126_0)
        let s_126_1: () = AArch32_TakeVirtualFIQException(state, tracer, s_126_0);
        // N s_126_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var interrupt_taken <= s_127_0
        fn_state.interrupt_taken = s_127_0;
        // N s_127_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_128_0: const #1u : u8
        let s_128_0: bool = true;
        // D s_128_1: write-var gs#327606 <= s_128_0
        fn_state.gs_327606 = s_128_0;
        // N s_128_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_129_0: const #1u : u8
        let s_129_0: bool = true;
        // D s_129_1: write-var gs#327605 <= s_129_0
        fn_state.gs_327605 = s_129_0;
        // N s_129_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_130_0: const #1u : u8
        let s_130_0: bool = true;
        // D s_130_1: write-var gs#327604 <= s_130_0
        fn_state.gs_327604 = s_130_0;
        // N s_130_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#327603 <= s_131_0
        fn_state.gs_327603 = s_131_0;
        // N s_131_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // D s_132_1: write-var gs#327602 <= s_132_0
        fn_state.gs_327602 = s_132_0;
        // N s_132_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var vFIQ <= s_133_0
        fn_state.vFIQ = s_133_0;
        // N s_133_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var FIQ <= s_134_0
        fn_state.FIQ = s_134_0;
        // N s_134_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var vIRQ <= s_135_0
        fn_state.vIRQ = s_135_0;
        // N s_135_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var IRQ <= s_136_0
        fn_state.IRQ = s_136_0;
        // N s_136_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var vAA <= s_137_0
        fn_state.vAA = s_137_0;
        // N s_137_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var AA <= s_138_0
        fn_state.AA = s_138_0;
        // N s_138_2: jump b92
        return block_92(state, tracer, fn_state);
    }
}
