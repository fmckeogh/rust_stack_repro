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
use HaveECVExt::*;
use u_get_CNTHPS_CTL_EL2_Type_ENABLE::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use u_get_CNTHCTL_EL2_Type_CNTPMASK::*;
use HaveSecureEL2Ext::*;
use HaveRME::*;
use u_get_CNTHPS_CTL_EL2_Type_IMASK::*;
use u_get_CNTP_CTL_EL0_Type_ENABLE::*;
use u_get_CNTHVS_CTL_EL2_Type_IMASK::*;
use IsTimerConditionMet::*;
use CurrentSecurityState::*;
use Zeros::*;
use u_get_CNTPS_CTL_EL1_Type_ENABLE::*;
use u_get_CNTHVS_CTL_EL2_Type_ENABLE::*;
use u_get_CNTV_CTL_EL0_Type_IMASK::*;
use u_get_CNTHCTL_EL2_Type_CNTVMASK::*;
use u_get_CNTHP_CTL_EL2_Type_IMASK::*;
use u_get_CNTP_CTL_EL0_Type_IMASK::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u_get_CNTPS_CTL_EL1_Type_IMASK::*;
use u_get_CNTHP_CTL_EL2_Type_ENABLE::*;
use u_get_CNTV_CTL_EL0_Type_ENABLE::*;
use EL2Enabled::*;
use u_get_CNTHV_CTL_EL2_Type_IMASK::*;
use HaveVirtHostExt::*;
use u_get_CNTHV_CTL_EL2_Type_ENABLE::*;
use common::*;
pub fn AArch64_CheckTimerConditions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_12456: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_12479: bool,
        gs_12508: bool,
        ecv: bool,
        gs_12463: bool,
        gs_12464: bool,
        gs_12457: bool,
        gs_12465: bool,
        imask: bool,
        gs_12459: bool,
        gs_12466: bool,
        gs_12460: bool,
        gs_12462: bool,
        gs_12458: bool,
        gs_12506: bool,
        gs_12507: bool,
        gs_12519: bool,
        gs_12518: bool,
        status: bool,
        ss: u32,
        gs_12481: bool,
        gs_12480: bool,
        offset: u64,
        gs_12461: bool,
        gs_12456: (),
    }
    let fn_state = FunctionState {
        gs_12456,
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
        // S s_0_1: call CurrentSecurityState(s_0_0)
        let s_0_1: u32 = CurrentSecurityState(state, tracer, s_0_0);
        // D s_0_2: write-var ss <= s_0_1
        fn_state.ss = s_0_1;
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // D s_0_4: write-var ecv <= s_0_3
        fn_state.ecv = s_0_3;
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call HaveECVExt(s_0_5)
        let s_0_6: bool = HaveECVExt(state, tracer, s_0_5);
        // N s_0_7: branch s_0_6 b101 b1
        if s_0_6 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ecv:u8
        let s_2_0: bool = fn_state.ecv;
        // N s_2_1: branch s_2_0 b100 b3
        if s_2_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // S s_3_1: call Zeros(s_3_0)
        let s_3_1: Bits = Zeros(state, tracer, s_3_0);
        // S s_3_2: cast reint s_3_1 -> u64
        let s_3_2: u64 = (s_3_1.value() as u64);
        // D s_3_3: write-var offset <= s_3_2
        fn_state.offset = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #90832u : u32
        let s_4_0: u32 = 90832;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_CNTP_CTL_EL0_Type_ENABLE(s_4_1)
        let s_4_2: bool = u_get_CNTP_CTL_EL0_Type_ENABLE(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // N s_4_7: branch s_4_6 b84 b5
        if s_4_6 {
            return block_84(state, tracer, fn_state);
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
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // N s_6_4: branch s_6_3 b83 b7
        if s_6_3 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #432u : u32
        let s_7_0: u32 = 432;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // D s_7_3: cmp-lt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) < (s_7_2));
        // N s_7_4: branch s_7_3 b82 b8
        if s_7_3 {
            return block_82(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#12457 <= s_8_0
        fn_state.gs_12457 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#12457:u8
        let s_9_0: bool = fn_state.gs_12457;
        // D s_9_1: write-var gs#12458 <= s_9_0
        fn_state.gs_12458 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#12458:u8
        let s_10_0: bool = fn_state.gs_12458;
        // N s_10_1: branch s_10_0 b81 b11
        if s_10_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#12459 <= s_11_0
        fn_state.gs_12459 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#12459:u8
        let s_12_0: bool = fn_state.gs_12459;
        // N s_12_1: branch s_12_0 b77 b13
        if s_12_0 {
            return block_77(state, tracer, fn_state);
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
        // C s_14_0: const #432u : u32
        let s_14_0: u32 = 432;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // D s_14_3: cmp-lt s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) < (s_14_2));
        // N s_14_4: branch s_14_3 b76 b15
        if s_14_3 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#12460 <= s_15_0
        fn_state.gs_12460 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#12460:u8
        let s_16_0: bool = fn_state.gs_12460;
        // N s_16_1: branch s_16_0 b75 b17
        if s_16_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#12461 <= s_17_0
        fn_state.gs_12461 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#12461:u8
        let s_18_0: bool = fn_state.gs_12461;
        // N s_18_1: branch s_18_0 b71 b19
        if s_18_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #20376u : u32
        let s_20_0: u32 = 20376;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_CNTPS_CTL_EL1_Type_ENABLE(s_20_1)
        let s_20_2: bool = u_get_CNTPS_CTL_EL1_Type_ENABLE(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #1u : u8
        let s_20_4: bool = true;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // N s_20_7: branch s_20_6 b67 b21
        if s_20_6 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #17200u : u32
        let s_22_0: u32 = 17200;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_CNTV_CTL_EL0_Type_ENABLE(s_22_1)
        let s_22_2: bool = u_get_CNTV_CTL_EL0_Type_ENABLE(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // N s_22_7: branch s_22_6 b51 b23
        if s_22_6 {
            return block_51(state, tracer, fn_state);
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
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveVirtHostExt(s_24_0)
        let s_24_1: bool = HaveVirtHostExt(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b47 b25
        if s_24_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#12463 <= s_25_0
        fn_state.gs_12463 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#12463:u8
        let s_26_0: bool = fn_state.gs_12463;
        // N s_26_1: branch s_26_0 b46 b27
        if s_26_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#12464 <= s_27_0
        fn_state.gs_12464 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#12464:u8
        let s_28_0: bool = fn_state.gs_12464;
        // N s_28_1: branch s_28_0 b42 b29
        if s_28_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call HaveSecureEL2Ext(s_30_0)
        let s_30_1: bool = HaveSecureEL2Ext(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b41 b31
        if s_30_1 {
            return block_41(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#12465 <= s_31_0
        fn_state.gs_12465 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#12465:u8
        let s_32_0: bool = fn_state.gs_12465;
        // N s_32_1: branch s_32_0 b40 b33
        if s_32_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#12466 <= s_33_0
        fn_state.gs_12466 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#12466:u8
        let s_34_0: bool = fn_state.gs_12466;
        // N s_34_1: branch s_34_0 b36 b35
        if s_34_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #64s : i
        let s_36_0: i128 = 64;
        // S s_36_1: call Zeros(s_36_0)
        let s_36_1: Bits = Zeros(state, tracer, s_36_0);
        // S s_36_2: cast reint s_36_1 -> u64
        let s_36_2: u64 = (s_36_1.value() as u64);
        // C s_36_3: const #10064u : u32
        let s_36_3: u32 = 10064;
        // D s_36_4: read-reg s_36_3:u64
        let s_36_4: u64 = {
            let value = state.read_register::<u64>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // C s_36_5: const #14872u : u32
        let s_36_5: u32 = 14872;
        // D s_36_6: read-reg s_36_5:struct
        let s_36_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_5 as isize);
            tracer.read_register(s_36_5 as isize, value);
            value
        };
        // D s_36_7: call _get_CNTHVS_CTL_EL2_Type_IMASK(s_36_6)
        let s_36_7: bool = u_get_CNTHVS_CTL_EL2_Type_IMASK(state, tracer, s_36_6);
        // C s_36_8: const #11u : u32
        let s_36_8: u32 = 11;
        // D s_36_9: call IsTimerConditionMet(s_36_2, s_36_4, s_36_7, s_36_8)
        let s_36_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_36_2,
            s_36_4,
            s_36_7,
            s_36_8,
        );
        // N s_36_10: branch s_36_9 b39 b37
        if s_36_9 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #14872u : u32
        let s_38_0: u32 = 14872;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // C s_38_2: const #14872u : u32
        let s_38_2: u32 = 14872;
        // N s_38_3: write-reg s_38_2 <= s_38_1
        let s_38_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_38_2 as isize, s_38_1);
            tracer.write_register(s_38_2 as isize, s_38_1);
        };
        // N s_38_4: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #14872u : u32
        let s_40_0: u32 = 14872;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_CNTHVS_CTL_EL2_Type_ENABLE(s_40_1)
        let s_40_2: bool = u_get_CNTHVS_CTL_EL2_Type_ENABLE(state, tracer, s_40_1);
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // C s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 1u16);
        // D s_40_6: cmp-eq s_40_3 s_40_5
        let s_40_6: bool = ((s_40_3) == (s_40_5));
        // D s_40_7: write-var gs#12466 <= s_40_6
        fn_state.gs_12466 = s_40_6;
        // N s_40_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call HaveVirtHostExt(s_41_0)
        let s_41_1: bool = HaveVirtHostExt(state, tracer, s_41_0);
        // D s_41_2: write-var gs#12465 <= s_41_1
        fn_state.gs_12465 = s_41_1;
        // N s_41_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #64s : i
        let s_42_0: i128 = 64;
        // S s_42_1: call Zeros(s_42_0)
        let s_42_1: Bits = Zeros(state, tracer, s_42_0);
        // S s_42_2: cast reint s_42_1 -> u64
        let s_42_2: u64 = (s_42_1.value() as u64);
        // C s_42_3: const #103152u : u32
        let s_42_3: u32 = 103152;
        // D s_42_4: read-reg s_42_3:u64
        let s_42_4: u64 = {
            let value = state.read_register::<u64>(s_42_3 as isize);
            tracer.read_register(s_42_3 as isize, value);
            value
        };
        // C s_42_5: const #19280u : u32
        let s_42_5: u32 = 19280;
        // D s_42_6: read-reg s_42_5:struct
        let s_42_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call _get_CNTHV_CTL_EL2_Type_IMASK(s_42_6)
        let s_42_7: bool = u_get_CNTHV_CTL_EL2_Type_IMASK(state, tracer, s_42_6);
        // C s_42_8: const #10u : u32
        let s_42_8: u32 = 10;
        // D s_42_9: call IsTimerConditionMet(s_42_2, s_42_4, s_42_7, s_42_8)
        let s_42_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_42_2,
            s_42_4,
            s_42_7,
            s_42_8,
        );
        // D s_42_10: write-var status <= s_42_9
        fn_state.status = s_42_9;
        // D s_42_11: read-var status:u8
        let s_42_11: bool = fn_state.status;
        // N s_42_12: branch s_42_11 b45 b43
        if s_42_11 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #19280u : u32
        let s_44_0: u32 = 19280;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // C s_44_2: const #19280u : u32
        let s_44_2: u32 = 19280;
        // N s_44_3: write-reg s_44_2 <= s_44_1
        let s_44_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_44_2 as isize, s_44_1);
            tracer.write_register(s_44_2 as isize, s_44_1);
        };
        // N s_44_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #19280u : u32
        let s_46_0: u32 = 19280;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_CNTHV_CTL_EL2_Type_ENABLE(s_46_1)
        let s_46_2: bool = u_get_CNTHV_CTL_EL2_Type_ENABLE(state, tracer, s_46_1);
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #1u : u8
        let s_46_4: bool = true;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // D s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var gs#12464 <= s_46_6
        fn_state.gs_12464 = s_46_6;
        // N s_46_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // C s_47_2: const #2u : u8
        let s_47_2: u8 = 2;
        // D s_47_3: cmp-lt s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) < (s_47_2));
        // N s_47_4: branch s_47_3 b50 b48
        if s_47_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call HaveSecureEL2Ext(s_48_0)
        let s_48_1: bool = HaveSecureEL2Ext(state, tracer, s_48_0);
        // S s_48_2: not s_48_1
        let s_48_2: bool = !s_48_1;
        // D s_48_3: write-var gs#12462 <= s_48_2
        fn_state.gs_12462 = s_48_2;
        // N s_48_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#12462:u8
        let s_49_0: bool = fn_state.gs_12462;
        // D s_49_1: write-var gs#12463 <= s_49_0
        fn_state.gs_12463 = s_49_0;
        // N s_49_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#12462 <= s_50_0
        fn_state.gs_12462 = s_50_0;
        // N s_50_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #17200u : u32
        let s_51_0: u32 = 17200;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_CNTV_CTL_EL0_Type_IMASK(s_51_1)
        let s_51_2: bool = u_get_CNTV_CTL_EL0_Type_IMASK(state, tracer, s_51_1);
        // D s_51_3: write-var imask <= s_51_2
        fn_state.imask = s_51_2;
        // C s_51_4: const #() : ()
        let s_51_4: () = ();
        // S s_51_5: call HaveRME(s_51_4)
        let s_51_5: bool = HaveRME(state, tracer, s_51_4);
        // N s_51_6: branch s_51_5 b63 b52
        if s_51_5 {
            return block_63(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#12480 <= s_52_0
        fn_state.gs_12480 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#12480:u8
        let s_53_0: bool = fn_state.gs_12480;
        // N s_53_1: branch s_53_0 b62 b54
        if s_53_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#12481 <= s_54_0
        fn_state.gs_12481 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#12481:u8
        let s_55_0: bool = fn_state.gs_12481;
        // N s_55_1: branch s_55_0 b61 b56
        if s_55_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #23632u : u32
        let s_57_0: u32 = 23632;
        // D s_57_1: read-reg s_57_0:u64
        let s_57_1: u64 = {
            let value = state.read_register::<u64>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #22400u : u32
        let s_57_2: u32 = 22400;
        // D s_57_3: read-reg s_57_2:u64
        let s_57_3: u64 = {
            let value = state.read_register::<u64>(s_57_2 as isize);
            tracer.read_register(s_57_2 as isize, value);
            value
        };
        // D s_57_4: read-var imask:u8
        let s_57_4: bool = fn_state.imask;
        // C s_57_5: const #9u : u32
        let s_57_5: u32 = 9;
        // D s_57_6: call IsTimerConditionMet(s_57_3, s_57_1, s_57_4, s_57_5)
        let s_57_6: bool = IsTimerConditionMet(
            state,
            tracer,
            s_57_3,
            s_57_1,
            s_57_4,
            s_57_5,
        );
        // D s_57_7: write-var status <= s_57_6
        fn_state.status = s_57_6;
        // D s_57_8: read-var status:u8
        let s_57_8: bool = fn_state.status;
        // N s_57_9: branch s_57_8 b60 b58
        if s_57_8 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #17200u : u32
        let s_59_0: u32 = 17200;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #17200u : u32
        let s_59_2: u32 = 17200;
        // N s_59_3: write-reg s_59_2 <= s_59_1
        let s_59_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_59_2 as isize, s_59_1);
            tracer.write_register(s_59_2 as isize, s_59_1);
        };
        // N s_59_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var imask <= s_61_0
        fn_state.imask = s_61_0;
        // N s_61_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #12808u : u32
        let s_62_0: u32 = 12808;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_CNTHCTL_EL2_Type_CNTVMASK(s_62_1)
        let s_62_2: bool = u_get_CNTHCTL_EL2_Type_CNTVMASK(state, tracer, s_62_1);
        // D s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // C s_62_4: const #1u : u8
        let s_62_4: bool = true;
        // C s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 1u16);
        // D s_62_6: cmp-eq s_62_3 s_62_5
        let s_62_6: bool = ((s_62_3) == (s_62_5));
        // D s_62_7: write-var gs#12481 <= s_62_6
        fn_state.gs_12481 = s_62_6;
        // N s_62_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var ss:u32
        let s_63_0: u32 = fn_state.ss;
        // C s_63_1: const #1u : u32
        let s_63_1: u32 = 1;
        // D s_63_2: cmp-eq s_63_0 s_63_1
        let s_63_2: bool = ((s_63_0) == (s_63_1));
        // N s_63_3: branch s_63_2 b66 b64
        if s_63_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var ss:u32
        let s_64_0: u32 = fn_state.ss;
        // C s_64_1: const #2u : u32
        let s_64_1: u32 = 2;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // D s_64_3: write-var gs#12479 <= s_64_2
        fn_state.gs_12479 = s_64_2;
        // N s_64_4: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#12479:u8
        let s_65_0: bool = fn_state.gs_12479;
        // D s_65_1: write-var gs#12480 <= s_65_0
        fn_state.gs_12480 = s_65_0;
        // N s_65_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#12479 <= s_66_0
        fn_state.gs_12479 = s_66_0;
        // N s_66_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #14160u : u32
        let s_67_0: u32 = 14160;
        // D s_67_1: read-reg s_67_0:u64
        let s_67_1: u64 = {
            let value = state.read_register::<u64>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // C s_67_2: const #20376u : u32
        let s_67_2: u32 = 20376;
        // D s_67_3: read-reg s_67_2:struct
        let s_67_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_67_2 as isize);
            tracer.read_register(s_67_2 as isize, value);
            value
        };
        // D s_67_4: call _get_CNTPS_CTL_EL1_Type_IMASK(s_67_3)
        let s_67_4: bool = u_get_CNTPS_CTL_EL1_Type_IMASK(state, tracer, s_67_3);
        // D s_67_5: read-var offset:u64
        let s_67_5: u64 = fn_state.offset;
        // C s_67_6: const #8u : u32
        let s_67_6: u32 = 8;
        // D s_67_7: call IsTimerConditionMet(s_67_5, s_67_1, s_67_4, s_67_6)
        let s_67_7: bool = IsTimerConditionMet(
            state,
            tracer,
            s_67_5,
            s_67_1,
            s_67_4,
            s_67_6,
        );
        // D s_67_8: write-var status <= s_67_7
        fn_state.status = s_67_7;
        // D s_67_9: read-var status:u8
        let s_67_9: bool = fn_state.status;
        // N s_67_10: branch s_67_9 b70 b68
        if s_67_9 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #20376u : u32
        let s_69_0: u32 = 20376;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // C s_69_2: const #20376u : u32
        let s_69_2: u32 = 20376;
        // N s_69_3: write-reg s_69_2 <= s_69_1
        let s_69_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_69_2 as isize, s_69_1);
            tracer.write_register(s_69_2 as isize, s_69_1);
        };
        // N s_69_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #64s : i
        let s_71_0: i128 = 64;
        // S s_71_1: call Zeros(s_71_0)
        let s_71_1: Bits = Zeros(state, tracer, s_71_0);
        // S s_71_2: cast reint s_71_1 -> u64
        let s_71_2: u64 = (s_71_1.value() as u64);
        // C s_71_3: const #22672u : u32
        let s_71_3: u32 = 22672;
        // D s_71_4: read-reg s_71_3:u64
        let s_71_4: u64 = {
            let value = state.read_register::<u64>(s_71_3 as isize);
            tracer.read_register(s_71_3 as isize, value);
            value
        };
        // C s_71_5: const #10504u : u32
        let s_71_5: u32 = 10504;
        // D s_71_6: read-reg s_71_5:struct
        let s_71_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call _get_CNTHPS_CTL_EL2_Type_IMASK(s_71_6)
        let s_71_7: bool = u_get_CNTHPS_CTL_EL2_Type_IMASK(state, tracer, s_71_6);
        // C s_71_8: const #7u : u32
        let s_71_8: u32 = 7;
        // D s_71_9: call IsTimerConditionMet(s_71_2, s_71_4, s_71_7, s_71_8)
        let s_71_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_71_2,
            s_71_4,
            s_71_7,
            s_71_8,
        );
        // D s_71_10: write-var status <= s_71_9
        fn_state.status = s_71_9;
        // D s_71_11: read-var status:u8
        let s_71_11: bool = fn_state.status;
        // N s_71_12: branch s_71_11 b74 b72
        if s_71_11 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #10504u : u32
        let s_73_0: u32 = 10504;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #10504u : u32
        let s_73_2: u32 = 10504;
        // N s_73_3: write-reg s_73_2 <= s_73_1
        let s_73_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_73_2 as isize, s_73_1);
            tracer.write_register(s_73_2 as isize, s_73_1);
        };
        // N s_73_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #10504u : u32
        let s_75_0: u32 = 10504;
        // D s_75_1: read-reg s_75_0:struct
        let s_75_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // D s_75_2: call _get_CNTHPS_CTL_EL2_Type_ENABLE(s_75_1)
        let s_75_2: bool = u_get_CNTHPS_CTL_EL2_Type_ENABLE(state, tracer, s_75_1);
        // D s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // C s_75_4: const #1u : u8
        let s_75_4: bool = true;
        // C s_75_5: cast zx s_75_4 -> bv
        let s_75_5: Bits = Bits::new(s_75_4 as u128, 1u16);
        // D s_75_6: cmp-eq s_75_3 s_75_5
        let s_75_6: bool = ((s_75_3) == (s_75_5));
        // D s_75_7: write-var gs#12461 <= s_75_6
        fn_state.gs_12461 = s_75_6;
        // N s_75_8: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call HaveSecureEL2Ext(s_76_0)
        let s_76_1: bool = HaveSecureEL2Ext(state, tracer, s_76_0);
        // D s_76_2: write-var gs#12460 <= s_76_1
        fn_state.gs_12460 = s_76_1;
        // N s_76_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #64s : i
        let s_77_0: i128 = 64;
        // S s_77_1: call Zeros(s_77_0)
        let s_77_1: Bits = Zeros(state, tracer, s_77_0);
        // S s_77_2: cast reint s_77_1 -> u64
        let s_77_2: u64 = (s_77_1.value() as u64);
        // C s_77_3: const #16640u : u32
        let s_77_3: u32 = 16640;
        // D s_77_4: read-reg s_77_3:u64
        let s_77_4: u64 = {
            let value = state.read_register::<u64>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // C s_77_5: const #100912u : u32
        let s_77_5: u32 = 100912;
        // D s_77_6: read-reg s_77_5:struct
        let s_77_6: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call _get_CNTHP_CTL_EL2_Type_IMASK(s_77_6)
        let s_77_7: bool = u_get_CNTHP_CTL_EL2_Type_IMASK(state, tracer, s_77_6);
        // C s_77_8: const #6u : u32
        let s_77_8: u32 = 6;
        // D s_77_9: call IsTimerConditionMet(s_77_2, s_77_4, s_77_7, s_77_8)
        let s_77_9: bool = IsTimerConditionMet(
            state,
            tracer,
            s_77_2,
            s_77_4,
            s_77_7,
            s_77_8,
        );
        // D s_77_10: write-var status <= s_77_9
        fn_state.status = s_77_9;
        // D s_77_11: read-var status:u8
        let s_77_11: bool = fn_state.status;
        // N s_77_12: branch s_77_11 b80 b78
        if s_77_11 {
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
        // N s_78_0: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #100912u : u32
        let s_79_0: u32 = 100912;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // C s_79_2: const #100912u : u32
        let s_79_2: u32 = 100912;
        // N s_79_3: write-reg s_79_2 <= s_79_1
        let s_79_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_79_2 as isize, s_79_1);
            tracer.write_register(s_79_2 as isize, s_79_1);
        };
        // N s_79_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_80_0: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #100912u : u32
        let s_81_0: u32 = 100912;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_CNTHP_CTL_EL2_Type_ENABLE(s_81_1)
        let s_81_2: bool = u_get_CNTHP_CTL_EL2_Type_ENABLE(state, tracer, s_81_1);
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #1u : u8
        let s_81_4: bool = true;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // D s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#12459 <= s_81_6
        fn_state.gs_12459 = s_81_6;
        // N s_81_8: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call HaveSecureEL2Ext(s_82_0)
        let s_82_1: bool = HaveSecureEL2Ext(state, tracer, s_82_0);
        // S s_82_2: not s_82_1
        let s_82_2: bool = !s_82_1;
        // D s_82_3: write-var gs#12457 <= s_82_2
        fn_state.gs_12457 = s_82_2;
        // N s_82_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1u : u8
        let s_83_0: bool = true;
        // D s_83_1: write-var gs#12458 <= s_83_0
        fn_state.gs_12458 = s_83_0;
        // N s_83_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #90832u : u32
        let s_84_0: u32 = 90832;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_CNTP_CTL_EL0_Type_IMASK(s_84_1)
        let s_84_2: bool = u_get_CNTP_CTL_EL0_Type_IMASK(state, tracer, s_84_1);
        // D s_84_3: write-var imask <= s_84_2
        fn_state.imask = s_84_2;
        // C s_84_4: const #() : ()
        let s_84_4: () = ();
        // S s_84_5: call HaveRME(s_84_4)
        let s_84_5: bool = HaveRME(state, tracer, s_84_4);
        // N s_84_6: branch s_84_5 b96 b85
        if s_84_5 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#12507 <= s_85_0
        fn_state.gs_12507 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#12507:u8
        let s_86_0: bool = fn_state.gs_12507;
        // N s_86_1: branch s_86_0 b95 b87
        if s_86_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#12508 <= s_87_0
        fn_state.gs_12508 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#12508:u8
        let s_88_0: bool = fn_state.gs_12508;
        // N s_88_1: branch s_88_0 b94 b89
        if s_88_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_89_0: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #20800u : u32
        let s_90_0: u32 = 20800;
        // D s_90_1: read-reg s_90_0:u64
        let s_90_1: u64 = {
            let value = state.read_register::<u64>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: read-var offset:u64
        let s_90_2: u64 = fn_state.offset;
        // D s_90_3: read-var imask:u8
        let s_90_3: bool = fn_state.imask;
        // C s_90_4: const #5u : u32
        let s_90_4: u32 = 5;
        // D s_90_5: call IsTimerConditionMet(s_90_2, s_90_1, s_90_3, s_90_4)
        let s_90_5: bool = IsTimerConditionMet(
            state,
            tracer,
            s_90_2,
            s_90_1,
            s_90_3,
            s_90_4,
        );
        // D s_90_6: write-var status <= s_90_5
        fn_state.status = s_90_5;
        // D s_90_7: read-var status:u8
        let s_90_7: bool = fn_state.status;
        // N s_90_8: branch s_90_7 b93 b91
        if s_90_7 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #90832u : u32
        let s_92_0: u32 = 90832;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #90832u : u32
        let s_92_2: u32 = 90832;
        // N s_92_3: write-reg s_92_2 <= s_92_1
        let s_92_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_92_2 as isize, s_92_1);
            tracer.write_register(s_92_2 as isize, s_92_1);
        };
        // N s_92_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_93_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1u : u8
        let s_94_0: bool = true;
        // D s_94_1: write-var imask <= s_94_0
        fn_state.imask = s_94_0;
        // N s_94_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #12808u : u32
        let s_95_0: u32 = 12808;
        // D s_95_1: read-reg s_95_0:struct
        let s_95_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_95_0 as isize);
            tracer.read_register(s_95_0 as isize, value);
            value
        };
        // D s_95_2: call _get_CNTHCTL_EL2_Type_CNTPMASK(s_95_1)
        let s_95_2: bool = u_get_CNTHCTL_EL2_Type_CNTPMASK(state, tracer, s_95_1);
        // D s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // C s_95_4: const #1u : u8
        let s_95_4: bool = true;
        // C s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 1u16);
        // D s_95_6: cmp-eq s_95_3 s_95_5
        let s_95_6: bool = ((s_95_3) == (s_95_5));
        // D s_95_7: write-var gs#12508 <= s_95_6
        fn_state.gs_12508 = s_95_6;
        // N s_95_8: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var ss:u32
        let s_96_0: u32 = fn_state.ss;
        // C s_96_1: const #1u : u32
        let s_96_1: u32 = 1;
        // D s_96_2: cmp-eq s_96_0 s_96_1
        let s_96_2: bool = ((s_96_0) == (s_96_1));
        // N s_96_3: branch s_96_2 b99 b97
        if s_96_2 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var ss:u32
        let s_97_0: u32 = fn_state.ss;
        // C s_97_1: const #2u : u32
        let s_97_1: u32 = 2;
        // D s_97_2: cmp-eq s_97_0 s_97_1
        let s_97_2: bool = ((s_97_0) == (s_97_1));
        // D s_97_3: write-var gs#12506 <= s_97_2
        fn_state.gs_12506 = s_97_2;
        // N s_97_4: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#12506:u8
        let s_98_0: bool = fn_state.gs_12506;
        // D s_98_1: write-var gs#12507 <= s_98_0
        fn_state.gs_12507 = s_98_0;
        // N s_98_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #1u : u8
        let s_99_0: bool = true;
        // D s_99_1: write-var gs#12506 <= s_99_0
        fn_state.gs_12506 = s_99_0;
        // N s_99_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #14584u : u32
        let s_100_0: u32 = 14584;
        // D s_100_1: read-reg s_100_0:u64
        let s_100_1: u64 = {
            let value = state.read_register::<u64>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: write-var offset <= s_100_1
        fn_state.offset = s_100_1;
        // N s_100_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #12808u : u32
        let s_101_0: u32 = 12808;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call _get_CNTHCTL_EL2_Type_ECV(s_101_1)
        let s_101_2: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_101_1);
        // D s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // C s_101_4: const #1u : u8
        let s_101_4: bool = true;
        // C s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 1u16);
        // D s_101_6: cmp-eq s_101_3 s_101_5
        let s_101_6: bool = ((s_101_3) == (s_101_5));
        // N s_101_7: branch s_101_6 b107 b102
        if s_101_6 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#12518 <= s_102_0
        fn_state.gs_12518 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#12518:u8
        let s_103_0: bool = fn_state.gs_12518;
        // N s_103_1: branch s_103_0 b106 b104
        if s_103_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#12519 <= s_104_0
        fn_state.gs_12519 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#12519:u8
        let s_105_0: bool = fn_state.gs_12519;
        // D s_105_1: write-var ecv <= s_105_0
        fn_state.ecv = s_105_0;
        // N s_105_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #() : ()
        let s_106_0: () = ();
        // S s_106_1: call EL2Enabled(s_106_0)
        let s_106_1: bool = EL2Enabled(state, tracer, s_106_0);
        // D s_106_2: write-var gs#12519 <= s_106_1
        fn_state.gs_12519 = s_106_1;
        // N s_106_3: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #90704u : u32
        let s_107_0: u32 = 90704;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_SCR_EL3_Type_ECVEn(s_107_1)
        let s_107_2: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_107_1);
        // D s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // C s_107_4: const #1u : u8
        let s_107_4: bool = true;
        // C s_107_5: cast zx s_107_4 -> bv
        let s_107_5: Bits = Bits::new(s_107_4 as u128, 1u16);
        // D s_107_6: cmp-eq s_107_3 s_107_5
        let s_107_6: bool = ((s_107_3) == (s_107_5));
        // D s_107_7: write-var gs#12518 <= s_107_6
        fn_state.gs_12518 = s_107_6;
        // N s_107_8: jump b103
        return block_103(state, tracer, fn_state);
    }
}
