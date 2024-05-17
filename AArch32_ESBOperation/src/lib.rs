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
use u_get_HCR_EL2_Type_AMO::*;
use ClearPendingPhysicalSError::*;
use Halted::*;
use HCR_read::*;
use CurrentSecurityState::*;
use ExternalDebugInterruptsDisabled::*;
use Zeros::*;
use Mk_DISR_Type::*;
use DISR_write::*;
use u_get_HCR_Type_TGE::*;
use AArch32_PhysicalSErrorSyndrome::*;
use u_get_SCR_Type_AW::*;
use IsSynchronizablePhysicalSErrorPending::*;
use ELFromM32::*;
use ELUsingAArch32::*;
use Bit::*;
use EffectiveEA::*;
use EL2Enabled::*;
use AArch64_ESBOperation::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_Type_AMO::*;
use common::*;
pub fn AArch32_ESBOperation<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_31663: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_monitor: bool,
        gs_31667: bool,
        gs_31686: bool,
        gs_31666: bool,
        gs_31681: bool,
        route_to_aarch64: bool,
        gs_31692: bool,
        mask_active: bool,
        gs_31668: bool,
        gs_31674: bool,
        gs_31679: bool,
        gs_31685: bool,
        el: u8,
        gs_31675: bool,
        target: u8,
        route_to_hyp: bool,
        gs_31665: bool,
        gs_31677: bool,
        gs_31664: bool,
        gs_31691: bool,
        gs_31683: bool,
        gs_31684: bool,
        gs_31673: bool,
        gs_31676: bool,
        gs_31693: bool,
        mask_set: bool,
        ga_24684: ProductTypea5cc8de4daab131c,
        gs_31690: bool,
        gs_31678: bool,
        gs_31663: (),
    }
    let fn_state = FunctionState {
        gs_31663,
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
        // N s_0_7: branch s_0_6 b83 b1
        if s_0_6 {
            return block_83(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#31664 <= s_1_0
        fn_state.gs_31664 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31664:u8
        let s_2_0: bool = fn_state.gs_31664;
        // D s_2_1: write-var route_to_aarch64 <= s_2_0
        fn_state.route_to_aarch64 = s_2_0;
        // D s_2_2: read-var route_to_aarch64:u8
        let s_2_2: bool = fn_state.route_to_aarch64;
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b82 b3
        if s_2_3 {
            return block_82(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#31665 <= s_3_0
        fn_state.gs_31665 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31665:u8
        let s_4_0: bool = fn_state.gs_31665;
        // N s_4_1: branch s_4_0 b81 b5
        if s_4_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#31666 <= s_5_0
        fn_state.gs_31666 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#31666:u8
        let s_6_0: bool = fn_state.gs_31666;
        // N s_6_1: branch s_6_0 b77 b7
        if s_6_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var route_to_aarch64:u8
        let s_8_0: bool = fn_state.route_to_aarch64;
        // D s_8_1: not s_8_0
        let s_8_1: bool = !s_8_0;
        // N s_8_2: branch s_8_1 b76 b9
        if s_8_1 {
            return block_76(state, tracer, fn_state);
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
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31667 <= s_9_0
        fn_state.gs_31667 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31667:u8
        let s_10_0: bool = fn_state.gs_31667;
        // N s_10_1: branch s_10_0 b75 b11
        if s_10_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#31668 <= s_11_0
        fn_state.gs_31668 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#31668:u8
        let s_12_0: bool = fn_state.gs_31668;
        // N s_12_1: branch s_12_0 b74 b13
        if s_12_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var route_to_aarch64:u8
        let s_14_0: bool = fn_state.route_to_aarch64;
        // N s_14_1: branch s_14_0 b73 b15
        if s_14_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // N s_15_4: branch s_15_3 b72 b16
        if s_15_3 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#31674 <= s_16_0
        fn_state.gs_31674 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#31674:u8
        let s_17_0: bool = fn_state.gs_31674;
        // N s_17_1: branch s_17_0 b71 b18
        if s_17_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#31675 <= s_18_0
        fn_state.gs_31675 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#31675:u8
        let s_19_0: bool = fn_state.gs_31675;
        // D s_19_1: write-var route_to_monitor <= s_19_0
        fn_state.route_to_monitor = s_19_0;
        // C s_19_2: const #16975u : u32
        let s_19_2: u32 = 16975;
        // D s_19_3: read-reg s_19_2:u8
        let s_19_3: u8 = {
            let value = state.read_register::<u8>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 2u16);
        // C s_19_5: const #448u : u32
        let s_19_5: u32 = 448;
        // D s_19_6: read-reg s_19_5:u8
        let s_19_6: u8 = {
            let value = state.read_register::<u8>(s_19_5 as isize);
            tracer.read_register(s_19_5 as isize, value);
            value
        };
        // D s_19_7: cast zx s_19_6 -> bv
        let s_19_7: Bits = Bits::new(s_19_6 as u128, 2u16);
        // D s_19_8: cmp-eq s_19_4 s_19_7
        let s_19_8: bool = ((s_19_4) == (s_19_7));
        // N s_19_9: branch s_19_8 b70 b20
        if s_19_8 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
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
        // C s_20_3: const #440u : u32
        let s_20_3: u32 = 440;
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
        // D s_20_7: write-var gs#31676 <= s_20_6
        fn_state.gs_31676 = s_20_6;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#31676:u8
        let s_21_0: bool = fn_state.gs_31676;
        // N s_21_1: branch s_21_0 b69 b22
        if s_21_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#31677 <= s_22_0
        fn_state.gs_31677 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#31677:u8
        let s_23_0: bool = fn_state.gs_31677;
        // N s_23_1: branch s_23_0 b65 b24
        if s_23_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#31679 <= s_24_0
        fn_state.gs_31679 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#31679:u8
        let s_25_0: bool = fn_state.gs_31679;
        // D s_25_1: write-var route_to_hyp <= s_25_0
        fn_state.route_to_hyp = s_25_0;
        // D s_25_2: read-var route_to_monitor:u8
        let s_25_2: bool = fn_state.route_to_monitor;
        // N s_25_3: branch s_25_2 b64 b26
        if s_25_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var route_to_hyp:u8
        let s_26_0: bool = fn_state.route_to_hyp;
        // N s_26_1: branch s_26_0 b63 b27
        if s_26_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #16983u : u32
        let s_27_0: u32 = 16983;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 5u16);
        // C s_27_3: const #400u : u32
        let s_27_3: u32 = 400;
        // D s_27_4: read-reg s_27_3:u8
        let s_27_4: u8 = {
            let value = state.read_register::<u8>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 5u16);
        // D s_27_6: cmp-eq s_27_2 s_27_5
        let s_27_6: bool = ((s_27_2) == (s_27_5));
        // D s_27_7: write-var gs#31681 <= s_27_6
        fn_state.gs_31681 = s_27_6;
        // N s_27_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#31681:u8
        let s_28_0: bool = fn_state.gs_31681;
        // N s_28_1: branch s_28_0 b62 b29
        if s_28_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #392u : u32
        let s_29_0: u32 = 392;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: write-var target <= s_29_1
        fn_state.target = s_29_1;
        // N s_29_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call CurrentSecurityState(s_30_0)
        let s_30_1: u32 = CurrentSecurityState(state, tracer, s_30_0);
        // C s_30_2: const #3u : u32
        let s_30_2: u32 = 3;
        // S s_30_3: cmp-eq s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) == (s_30_2));
        // N s_30_4: branch s_30_3 b61 b31
        if s_30_3 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var target:u8
        let s_31_0: u8 = fn_state.target;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 5u16);
        // C s_31_2: const #384u : u32
        let s_31_2: u32 = 384;
        // D s_31_3: read-reg s_31_2:u8
        let s_31_3: u8 = {
            let value = state.read_register::<u8>(s_31_2 as isize);
            tracer.read_register(s_31_2 as isize, value);
            value
        };
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 5u16);
        // D s_31_5: cmp-eq s_31_1 s_31_4
        let s_31_5: bool = ((s_31_1) == (s_31_4));
        // N s_31_6: branch s_31_5 b51 b32
        if s_31_5 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var target:u8
        let s_32_0: u8 = fn_state.target;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 5u16);
        // C s_32_2: const #392u : u32
        let s_32_2: u32 = 392;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 5u16);
        // D s_32_5: cmp-eq s_32_1 s_32_4
        let s_32_5: bool = ((s_32_1) == (s_32_4));
        // N s_32_6: branch s_32_5 b50 b33
        if s_32_5 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #16983u : u32
        let s_33_0: u32 = 16983;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: cast zx s_33_1 -> bv
        let s_33_2: Bits = Bits::new(s_33_1 as u128, 5u16);
        // C s_33_3: const #400u : u32
        let s_33_3: u32 = 400;
        // D s_33_4: read-reg s_33_3:u8
        let s_33_4: u8 = {
            let value = state.read_register::<u8>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 5u16);
        // D s_33_6: cmp-eq s_33_2 s_33_5
        let s_33_6: bool = ((s_33_2) == (s_33_5));
        // D s_33_7: write-var gs#31683 <= s_33_6
        fn_state.gs_31683 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#31683:u8
        let s_34_0: bool = fn_state.gs_31683;
        // D s_34_1: write-var mask_active <= s_34_0
        fn_state.mask_active = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #16968u : u32
        let s_35_0: u32 = 16968;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: bool = {
            let value = state.read_register::<bool>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 1u16);
        // C s_35_3: const #1u : u8
        let s_35_3: bool = true;
        // C s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 1u16);
        // D s_35_5: cmp-eq s_35_2 s_35_4
        let s_35_5: bool = ((s_35_2) == (s_35_4));
        // D s_35_6: write-var mask_set <= s_35_5
        fn_state.mask_set = s_35_5;
        // D s_35_7: read-var target:u8
        let s_35_7: u8 = fn_state.target;
        // D s_35_8: call ELFromM32(s_35_7)
        let s_35_8: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_35_7);
        // D s_35_9: write-var ga#24684 <= s_35_8
        fn_state.ga_24684 = s_35_8;
        // D s_35_10: read-var ga#24684.1:struct
        let s_35_10: u8 = fn_state.ga_24684._1;
        // D s_35_11: write-var el <= s_35_10
        fn_state.el = s_35_10;
        // C s_35_12: const #() : ()
        let s_35_12: () = ();
        // S s_35_13: call Halted(s_35_12)
        let s_35_13: bool = Halted(state, tracer, s_35_12);
        // N s_35_14: branch s_35_13 b49 b36
        if s_35_13 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var el:u8
        let s_36_0: u8 = fn_state.el;
        // D s_36_1: call ExternalDebugInterruptsDisabled(s_36_0)
        let s_36_1: bool = ExternalDebugInterruptsDisabled(state, tracer, s_36_0);
        // D s_36_2: write-var gs#31690 <= s_36_1
        fn_state.gs_31690 = s_36_1;
        // N s_36_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#31690:u8
        let s_37_0: bool = fn_state.gs_31690;
        // N s_37_1: branch s_37_0 b48 b38
        if s_37_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var mask_active:u8
        let s_38_0: bool = fn_state.mask_active;
        // N s_38_1: branch s_38_0 b47 b39
        if s_38_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#31691 <= s_39_0
        fn_state.gs_31691 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#31691:u8
        let s_40_0: bool = fn_state.gs_31691;
        // D s_40_1: write-var gs#31692 <= s_40_0
        fn_state.gs_31692 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#31692:u8
        let s_41_0: bool = fn_state.gs_31692;
        // N s_41_1: branch s_41_0 b46 b42
        if s_41_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#31693 <= s_42_0
        fn_state.gs_31693 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#31693:u8
        let s_43_0: bool = fn_state.gs_31693;
        // N s_43_1: branch s_43_0 b45 b44
        if s_43_0 {
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
        // N s_44_0: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #32s : i
        let s_45_0: i128 = 32;
        // S s_45_1: call Zeros(s_45_0)
        let s_45_1: Bits = Zeros(state, tracer, s_45_0);
        // S s_45_2: cast reint s_45_1 -> u32
        let s_45_2: u32 = (s_45_1.value() as u32);
        // C s_45_3: const #1u : u8
        let s_45_3: bool = true;
        // S s_45_4: call Bit(s_45_3)
        let s_45_4: bool = Bit(state, tracer, s_45_3);
        // C s_45_5: const #31s : i
        let s_45_5: i128 = 31;
        // S s_45_6: cast zx s_45_2 -> bv
        let s_45_6: Bits = Bits::new(s_45_2 as u128, 32u16);
        // C s_45_7: const #1u : u64
        let s_45_7: u64 = 1;
        // D s_45_8: bit-insert s_45_6 s_45_6 s_45_5 s_45_7
        let s_45_8: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_45_7 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_45_6.length(),
            );
            (s_45_6 & mask) | (s_45_6 << s_45_5)
        };
        // D s_45_9: cast reint s_45_8 -> u32
        let s_45_9: u32 = (s_45_8.value() as u32);
        // C s_45_10: const #() : ()
        let s_45_10: () = ();
        // S s_45_11: call AArch32_PhysicalSErrorSyndrome(s_45_10)
        let s_45_11: u16 = AArch32_PhysicalSErrorSyndrome(state, tracer, s_45_10);
        // C s_45_12: const #0s : i
        let s_45_12: i128 = 0;
        // D s_45_13: cast zx s_45_9 -> bv
        let s_45_13: Bits = Bits::new(s_45_9 as u128, 32u16);
        // S s_45_14: cast zx s_45_11 -> bv
        let s_45_14: Bits = Bits::new(s_45_11 as u128, 16u16);
        // C s_45_15: const #15s : i
        let s_45_15: i128 = 15;
        // C s_45_16: const #1u : u64
        let s_45_16: u64 = 1;
        // C s_45_17: cast zx s_45_16 -> bv
        let s_45_17: Bits = Bits::new(s_45_16 as u128, 64u16);
        // C s_45_18: lsl s_45_17 s_45_15
        let s_45_18: Bits = s_45_17 << s_45_15;
        // C s_45_19: sub s_45_18 s_45_17
        let s_45_19: Bits = ((s_45_18) - (s_45_17));
        // S s_45_20: and s_45_14 s_45_19
        let s_45_20: Bits = ((s_45_14) & (s_45_19));
        // S s_45_21: lsl s_45_20 s_45_12
        let s_45_21: Bits = s_45_20 << s_45_12;
        // C s_45_22: lsl s_45_19 s_45_12
        let s_45_22: Bits = s_45_19 << s_45_12;
        // C s_45_23: cmpl s_45_22
        let s_45_23: Bits = !s_45_22;
        // D s_45_24: and s_45_13 s_45_23
        let s_45_24: Bits = ((s_45_13) & (s_45_23));
        // D s_45_25: or s_45_24 s_45_21
        let s_45_25: Bits = ((s_45_24) | (s_45_21));
        // D s_45_26: cast reint s_45_25 -> u32
        let s_45_26: u32 = (s_45_25.value() as u32);
        // D s_45_27: call Mk_DISR_Type(s_45_26)
        let s_45_27: ProductType700c18a878c5601b = Mk_DISR_Type(state, tracer, s_45_26);
        // D s_45_28: call DISR_write(s_45_27)
        let s_45_28: () = DISR_write(state, tracer, s_45_27);
        // C s_45_29: const #() : ()
        let s_45_29: () = ();
        // S s_45_30: call ClearPendingPhysicalSError(s_45_29)
        let s_45_30: () = ClearPendingPhysicalSError(state, tracer, s_45_29);
        // N s_45_31: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call IsSynchronizablePhysicalSErrorPending(s_46_0)
        let s_46_1: bool = IsSynchronizablePhysicalSErrorPending(state, tracer, s_46_0);
        // D s_46_2: write-var gs#31693 <= s_46_1
        fn_state.gs_31693 = s_46_1;
        // N s_46_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var mask_set:u8
        let s_47_0: bool = fn_state.mask_set;
        // D s_47_1: write-var gs#31691 <= s_47_0
        fn_state.gs_31691 = s_47_0;
        // N s_47_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#31692 <= s_48_0
        fn_state.gs_31692 = s_48_0;
        // N s_48_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#31690 <= s_49_0
        fn_state.gs_31690 = s_49_0;
        // N s_49_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#31683 <= s_50_0
        fn_state.gs_31683 = s_50_0;
        // N s_50_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #20920u : u32
        let s_51_0: u32 = 20920;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_SCR_Type_AW(s_51_1)
        let s_51_2: bool = u_get_SCR_Type_AW(state, tracer, s_51_1);
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // C s_51_4: const #1u : u8
        let s_51_4: bool = true;
        // C s_51_5: cast zx s_51_4 -> bv
        let s_51_5: Bits = Bits::new(s_51_4 as u128, 1u16);
        // D s_51_6: cmp-eq s_51_3 s_51_5
        let s_51_6: bool = ((s_51_3) == (s_51_5));
        // N s_51_7: branch s_51_6 b54 b52
        if s_51_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#31686 <= s_52_0
        fn_state.gs_31686 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#31686:u8
        let s_53_0: bool = fn_state.gs_31686;
        // D s_53_1: write-var mask_active <= s_53_0
        fn_state.mask_active = s_53_0;
        // N s_53_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #432u : u32
        let s_54_0: u32 = 432;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: not s_54_3
        let s_54_4: bool = !s_54_3;
        // N s_54_5: branch s_54_4 b60 b55
        if s_54_4 {
            return block_60(state, tracer, fn_state);
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
        // S s_55_1: call HCR_read(s_55_0)
        let s_55_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_55_0);
        // S s_55_2: call _get_HCR_Type_TGE(s_55_1)
        let s_55_2: bool = u_get_HCR_Type_TGE(state, tracer, s_55_1);
        // S s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #0u : u8
        let s_55_4: bool = false;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // S s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // N s_55_7: branch s_55_6 b59 b56
        if s_55_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#31684 <= s_56_0
        fn_state.gs_31684 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#31684:u8
        let s_57_0: bool = fn_state.gs_31684;
        // D s_57_1: write-var gs#31685 <= s_57_0
        fn_state.gs_31685 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#31685:u8
        let s_58_0: bool = fn_state.gs_31685;
        // D s_58_1: write-var gs#31686 <= s_58_0
        fn_state.gs_31686 = s_58_0;
        // N s_58_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call HCR_read(s_59_0)
        let s_59_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_59_0);
        // S s_59_2: call _get_HCR_Type_AMO(s_59_1)
        let s_59_2: bool = u_get_HCR_Type_AMO(state, tracer, s_59_1);
        // S s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // C s_59_4: const #0u : u8
        let s_59_4: bool = false;
        // C s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 1u16);
        // S s_59_6: cmp-eq s_59_3 s_59_5
        let s_59_6: bool = ((s_59_3) == (s_59_5));
        // D s_59_7: write-var gs#31684 <= s_59_6
        fn_state.gs_31684 = s_59_6;
        // N s_59_8: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#31685 <= s_60_0
        fn_state.gs_31685 = s_60_0;
        // N s_60_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var mask_active <= s_61_0
        fn_state.mask_active = s_61_0;
        // N s_61_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #400u : u32
        let s_62_0: u32 = 400;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: write-var target <= s_62_1
        fn_state.target = s_62_1;
        // N s_62_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#31681 <= s_63_0
        fn_state.gs_31681 = s_63_0;
        // N s_63_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #384u : u32
        let s_64_0: u32 = 384;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: write-var target <= s_64_1
        fn_state.target = s_64_1;
        // N s_64_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call HCR_read(s_65_0)
        let s_65_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_65_0);
        // S s_65_2: call _get_HCR_Type_TGE(s_65_1)
        let s_65_2: bool = u_get_HCR_Type_TGE(state, tracer, s_65_1);
        // S s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // C s_65_4: const #1u : u8
        let s_65_4: bool = true;
        // C s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 1u16);
        // S s_65_6: cmp-eq s_65_3 s_65_5
        let s_65_6: bool = ((s_65_3) == (s_65_5));
        // N s_65_7: branch s_65_6 b68 b66
        if s_65_6 {
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
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call HCR_read(s_66_0)
        let s_66_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_66_0);
        // S s_66_2: call _get_HCR_Type_AMO(s_66_1)
        let s_66_2: bool = u_get_HCR_Type_AMO(state, tracer, s_66_1);
        // S s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // C s_66_4: const #1u : u8
        let s_66_4: bool = true;
        // C s_66_5: cast zx s_66_4 -> bv
        let s_66_5: Bits = Bits::new(s_66_4 as u128, 1u16);
        // S s_66_6: cmp-eq s_66_3 s_66_5
        let s_66_6: bool = ((s_66_3) == (s_66_5));
        // D s_66_7: write-var gs#31678 <= s_66_6
        fn_state.gs_31678 = s_66_6;
        // N s_66_8: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#31678:u8
        let s_67_0: bool = fn_state.gs_31678;
        // D s_67_1: write-var gs#31679 <= s_67_0
        fn_state.gs_31679 = s_67_0;
        // N s_67_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#31678 <= s_68_0
        fn_state.gs_31678 = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // D s_69_2: write-var gs#31677 <= s_69_1
        fn_state.gs_31677 = s_69_1;
        // N s_69_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#31676 <= s_70_0
        fn_state.gs_31676 = s_70_0;
        // N s_70_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EffectiveEA(s_71_0)
        let s_71_1: bool = EffectiveEA(state, tracer, s_71_0);
        // S s_71_2: cast zx s_71_1 -> bv
        let s_71_2: Bits = Bits::new(s_71_1 as u128, 1u16);
        // C s_71_3: const #1u : u8
        let s_71_3: bool = true;
        // C s_71_4: cast zx s_71_3 -> bv
        let s_71_4: Bits = Bits::new(s_71_3 as u128, 1u16);
        // S s_71_5: cmp-eq s_71_2 s_71_4
        let s_71_5: bool = ((s_71_2) == (s_71_4));
        // D s_71_6: write-var gs#31675 <= s_71_5
        fn_state.gs_31675 = s_71_5;
        // N s_71_7: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call ELUsingAArch32(s_72_1)
        let s_72_2: bool = ELUsingAArch32(state, tracer, s_72_1);
        // D s_72_3: write-var gs#31674 <= s_72_2
        fn_state.gs_31674 = s_72_2;
        // N s_72_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call AArch64_ESBOperation(s_73_0)
        let s_73_1: () = AArch64_ESBOperation(state, tracer, s_73_0);
        // N s_73_2: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call EffectiveEA(s_74_0)
        let s_74_1: bool = EffectiveEA(state, tracer, s_74_0);
        // S s_74_2: cast zx s_74_1 -> bv
        let s_74_2: Bits = Bits::new(s_74_1 as u128, 1u16);
        // C s_74_3: const #1u : u8
        let s_74_3: bool = true;
        // C s_74_4: cast zx s_74_3 -> bv
        let s_74_4: Bits = Bits::new(s_74_3 as u128, 1u16);
        // S s_74_5: cmp-eq s_74_2 s_74_4
        let s_74_5: bool = ((s_74_2) == (s_74_4));
        // D s_74_6: write-var route_to_aarch64 <= s_74_5
        fn_state.route_to_aarch64 = s_74_5;
        // N s_74_7: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #424u : u32
        let s_75_0: u32 = 424;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: u8 = {
            let value = state.read_register::<u8>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call ELUsingAArch32(s_75_1)
        let s_75_2: bool = ELUsingAArch32(state, tracer, s_75_1);
        // D s_75_3: not s_75_2
        let s_75_3: bool = !s_75_2;
        // D s_75_4: write-var gs#31668 <= s_75_3
        fn_state.gs_31668 = s_75_3;
        // N s_75_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #424u : u32
        let s_76_0: u32 = 424;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // C s_76_2: const #2u : u8
        let s_76_2: u8 = 2;
        // D s_76_3: cmp-lt s_76_1 s_76_2
        let s_76_3: bool = ((s_76_1) < (s_76_2));
        // D s_76_4: write-var gs#31667 <= s_76_3
        fn_state.gs_31667 = s_76_3;
        // N s_76_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #102552u : u32
        let s_77_0: u32 = 102552;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_HCR_EL2_Type_TGE(s_77_1)
        let s_77_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_77_1);
        // D s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // C s_77_4: const #1u : u8
        let s_77_4: bool = true;
        // C s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 1u16);
        // D s_77_6: cmp-eq s_77_3 s_77_5
        let s_77_6: bool = ((s_77_3) == (s_77_5));
        // N s_77_7: branch s_77_6 b80 b78
        if s_77_6 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #102552u : u32
        let s_78_0: u32 = 102552;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_HCR_EL2_Type_AMO(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_AMO(state, tracer, s_78_1);
        // D s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // C s_78_4: const #1u : u8
        let s_78_4: bool = true;
        // C s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 1u16);
        // D s_78_6: cmp-eq s_78_3 s_78_5
        let s_78_6: bool = ((s_78_3) == (s_78_5));
        // D s_78_7: write-var gs#31673 <= s_78_6
        fn_state.gs_31673 = s_78_6;
        // N s_78_8: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#31673:u8
        let s_79_0: bool = fn_state.gs_31673;
        // D s_79_1: write-var route_to_aarch64 <= s_79_0
        fn_state.route_to_aarch64 = s_79_0;
        // N s_79_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#31673 <= s_80_0
        fn_state.gs_31673 = s_80_0;
        // N s_80_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #432u : u32
        let s_81_0: u32 = 432;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // D s_81_4: write-var gs#31666 <= s_81_3
        fn_state.gs_31666 = s_81_3;
        // N s_81_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call EL2Enabled(s_82_0)
        let s_82_1: bool = EL2Enabled(state, tracer, s_82_0);
        // D s_82_2: write-var gs#31665 <= s_82_1
        fn_state.gs_31665 = s_82_1;
        // N s_82_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #440u : u32
        let s_83_0: u32 = 440;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#31664 <= s_83_3
        fn_state.gs_31664 = s_83_3;
        // N s_83_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
