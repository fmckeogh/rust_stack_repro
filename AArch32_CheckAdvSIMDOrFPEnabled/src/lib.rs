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
use AArch64_CheckFPAdvSIMDEnabled::*;
use ConstrainUnpredictableBool::*;
use AArch32_CheckFPAdvSIMDTrap::*;
use FPEXC_read::*;
use u_get_NSACR_Type_NSASEDIS::*;
use u_get_HCR_EL2_Type_RW::*;
use CurrentSecurityState::*;
use u_get_CPACR_Type_ASEDIS::*;
use u_get_FPEXC_Type_EN::*;
use ELUsingAArch32::*;
use u_get_CPACR_Type_cp10::*;
use u_get_NSACR_Type_cp10::*;
use u__IMPDEF_bits::*;
use AArch64_CheckFPEnabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use CPACR_read__1::*;
use common::*;
pub fn AArch32_CheckAdvSIMDOrFPEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fpexc_check: bool,
    advsimd: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cpacr_asedis: bool,
        gs_30948: bool,
        gs_30947: bool,
        gs_30940: bool,
        gs_30943: bool,
        gs_30944: bool,
        gs_30945: bool,
        gs_30941: bool,
        gs_30938: bool,
        gs_30942: bool,
        gs_30951: bool,
        disabled: bool,
        gs_30946: bool,
        gs_30965: bool,
        gs_30939: bool,
        cpacr_cp10: u8,
        fpexc_check: bool,
        advsimd: bool,
    }
    let fn_state = FunctionState {
        fpexc_check,
        advsimd,
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
        // N s_0_7: branch s_0_6 b74 b1
        if s_0_6 {
            return block_74(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#30938 <= s_1_0
        fn_state.gs_30938 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30938:u8
        let s_2_0: bool = fn_state.gs_30938;
        // N s_2_1: branch s_2_0 b67 b3
        if s_2_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#30941 <= s_3_0
        fn_state.gs_30941 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#30941:u8
        let s_4_0: bool = fn_state.gs_30941;
        // N s_4_1: branch s_4_0 b65 b5
        if s_4_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #448u : u32
        let s_5_3: u32 = 448;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // N s_5_7: branch s_5_6 b64 b6
        if s_5_6 {
            return block_64(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#30942 <= s_6_0
        fn_state.gs_30942 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#30942:u8
        let s_7_0: bool = fn_state.gs_30942;
        // N s_7_1: branch s_7_0 b63 b8
        if s_7_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#30943 <= s_8_0
        fn_state.gs_30943 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#30943:u8
        let s_9_0: bool = fn_state.gs_30943;
        // N s_9_1: branch s_9_0 b62 b10
        if s_9_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#30944 <= s_10_0
        fn_state.gs_30944 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#30944:u8
        let s_11_0: bool = fn_state.gs_30944;
        // N s_11_1: branch s_11_0 b61 b12
        if s_11_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#30945 <= s_12_0
        fn_state.gs_30945 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#30945:u8
        let s_13_0: bool = fn_state.gs_30945;
        // N s_13_1: branch s_13_0 b52 b14
        if s_13_0 {
            return block_52(state, tracer, fn_state);
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
        // S s_14_1: call CPACR_read__1(s_14_0)
        let s_14_1: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_14_0);
        // S s_14_2: call _get_CPACR_Type_ASEDIS(s_14_1)
        let s_14_2: bool = u_get_CPACR_Type_ASEDIS(state, tracer, s_14_1);
        // D s_14_3: write-var cpacr_asedis <= s_14_2
        fn_state.cpacr_asedis = s_14_2;
        // C s_14_4: const #() : ()
        let s_14_4: () = ();
        // S s_14_5: call CPACR_read__1(s_14_4)
        let s_14_5: ProductType700c18a878c5601b = CPACR_read__1(state, tracer, s_14_4);
        // S s_14_6: call _get_CPACR_Type_cp10(s_14_5)
        let s_14_6: u8 = u_get_CPACR_Type_cp10(state, tracer, s_14_5);
        // D s_14_7: write-var cpacr_cp10 <= s_14_6
        fn_state.cpacr_cp10 = s_14_6;
        // C s_14_8: const #424u : u32
        let s_14_8: u32 = 424;
        // D s_14_9: read-reg s_14_8:u8
        let s_14_9: u8 = {
            let value = state.read_register::<u8>(s_14_8 as isize);
            tracer.read_register(s_14_8 as isize, value);
            value
        };
        // C s_14_10: const #2u : u8
        let s_14_10: u8 = 2;
        // D s_14_11: cmp-lt s_14_9 s_14_10
        let s_14_11: bool = ((s_14_9) < (s_14_10));
        // N s_14_12: branch s_14_11 b51 b15
        if s_14_11 {
            return block_51(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#30946 <= s_15_0
        fn_state.gs_30946 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#30946:u8
        let s_16_0: bool = fn_state.gs_30946;
        // N s_16_1: branch s_16_0 b50 b17
        if s_16_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#30947 <= s_17_0
        fn_state.gs_30947 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#30947:u8
        let s_18_0: bool = fn_state.gs_30947;
        // N s_18_1: branch s_18_0 b43 b19
        if s_18_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_20_6: cmp-ne s_20_2 s_20_5
        let s_20_6: bool = ((s_20_2) != (s_20_5));
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
    ) -> () {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var fpexc_check:u8
        let s_22_0: bool = fn_state.fpexc_check;
        // N s_22_1: branch s_22_0 b27 b23
        if s_22_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#30948 <= s_23_0
        fn_state.gs_30948 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#30948:u8
        let s_24_0: bool = fn_state.gs_30948;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var advsimd:u8
        let s_25_0: bool = fn_state.advsimd;
        // D s_25_1: call AArch32_CheckFPAdvSIMDTrap(s_25_0)
        let s_25_1: () = AArch32_CheckFPAdvSIMDTrap(state, tracer, s_25_0);
        // N s_25_2: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call FPEXC_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = FPEXC_read(state, tracer, s_27_0);
        // S s_27_2: call _get_FPEXC_Type_EN(s_27_1)
        let s_27_2: bool = u_get_FPEXC_Type_EN(state, tracer, s_27_1);
        // S s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #0u : u8
        let s_27_4: bool = false;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // S s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#30948 <= s_27_6
        fn_state.gs_30948 = s_27_6;
        // N s_27_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var advsimd:u8
        let s_28_0: bool = fn_state.advsimd;
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
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#30951 <= s_29_0
        fn_state.gs_30951 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#30951:u8
        let s_30_0: bool = fn_state.gs_30951;
        // N s_30_1: branch s_30_0 b41 b31
        if s_30_0 {
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
        // D s_31_0: read-var cpacr_cp10:u8
        let s_31_0: u8 = fn_state.cpacr_cp10;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 2u16);
        // C s_31_2: const #0u : u8
        let s_31_2: u8 = 0;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 2u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b36 b32
        if s_31_5 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var disabled <= s_32_0
        fn_state.disabled = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var disabled:u8
        let s_33_0: bool = fn_state.disabled;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var cpacr_cp10:u8
        let s_36_0: u8 = fn_state.cpacr_cp10;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 2u16);
        // C s_36_2: const #1u : u8
        let s_36_2: u8 = 1;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 2u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #16975u : u32
        let s_37_0: u32 = 16975;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 2u16);
        // C s_37_3: const #448u : u32
        let s_37_3: u32 = 448;
        // D s_37_4: read-reg s_37_3:u8
        let s_37_4: u8 = {
            let value = state.read_register::<u8>(s_37_3 as isize);
            tracer.read_register(s_37_3 as isize, value);
            value
        };
        // D s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 2u16);
        // D s_37_6: cmp-eq s_37_2 s_37_5
        let s_37_6: bool = ((s_37_2) == (s_37_5));
        // D s_37_7: write-var disabled <= s_37_6
        fn_state.disabled = s_37_6;
        // N s_37_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var cpacr_cp10:u8
        let s_38_0: u8 = fn_state.cpacr_cp10;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 2u16);
        // C s_38_2: const #2u : u8
        let s_38_2: u8 = 2;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 2u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: not s_38_4
        let s_38_5: bool = !s_38_4;
        // N s_38_6: branch s_38_5 b40 b39
        if s_38_5 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #8u : u32
        let s_39_0: u32 = 8;
        // S s_39_1: call ConstrainUnpredictableBool(s_39_0)
        let s_39_1: bool = ConstrainUnpredictableBool(state, tracer, s_39_0);
        // D s_39_2: write-var disabled <= s_39_1
        fn_state.disabled = s_39_1;
        // N s_39_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var disabled <= s_40_0
        fn_state.disabled = s_40_0;
        // N s_40_2: jump b33
        return block_33(state, tracer, fn_state);
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
        // D s_42_0: read-var cpacr_asedis:u8
        let s_42_0: bool = fn_state.cpacr_asedis;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#30951 <= s_42_4
        fn_state.gs_30951 = s_42_4;
        // N s_42_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #102488u : u32
        let s_43_0: u32 = 102488;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_NSACR_Type_NSASEDIS(s_43_1)
        let s_43_2: bool = u_get_NSACR_Type_NSASEDIS(state, tracer, s_43_1);
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // D s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // N s_43_7: branch s_43_6 b49 b44
        if s_43_6 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #102488u : u32
        let s_45_0: u32 = 102488;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_NSACR_Type_cp10(s_45_1)
        let s_45_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_45_1);
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
    ) -> () {
        // N s_46_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: u8 = 0;
        // D s_48_1: write-var cpacr_cp10 <= s_48_0
        fn_state.cpacr_cp10 = s_48_0;
        // N s_48_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var cpacr_asedis <= s_49_0
        fn_state.cpacr_asedis = s_49_0;
        // N s_49_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call CurrentSecurityState(s_50_0)
        let s_50_1: u32 = CurrentSecurityState(state, tracer, s_50_0);
        // C s_50_2: const #0u : u32
        let s_50_2: u32 = 0;
        // S s_50_3: cmp-eq s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) == (s_50_2));
        // D s_50_4: write-var gs#30947 <= s_50_3
        fn_state.gs_30947 = s_50_3;
        // N s_50_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: write-var gs#30946 <= s_51_2
        fn_state.gs_30946 = s_51_2;
        // N s_51_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var fpexc_check:u8
        let s_52_0: bool = fn_state.fpexc_check;
        // N s_52_1: branch s_52_0 b60 b53
        if s_52_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#30965 <= s_53_0
        fn_state.gs_30965 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#30965:u8
        let s_54_0: bool = fn_state.gs_30965;
        // N s_54_1: branch s_54_0 b57 b55
        if s_54_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call AArch64_CheckFPEnabled(s_56_0)
        let s_56_1: () = AArch64_CheckFPEnabled(state, tracer, s_56_0);
        // N s_56_2: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1s : i64
        let s_57_0: i64 = 1;
        // C s_57_1: cast zx s_57_0 -> i
        let s_57_1: i128 = (i128::try_from(s_57_0).unwrap());
        // C s_57_2: const #"FPEXC.EN value when TGE==1 and RW==0" : str
        let s_57_2: &'static str = "FPEXC.EN value when TGE==1 and RW==0";
        // S s_57_3: call __IMPDEF_bits(s_57_1, s_57_2)
        let s_57_3: Bits = u__IMPDEF_bits(state, tracer, s_57_1, s_57_2);
        // S s_57_4: cast reint s_57_3 -> u8
        let s_57_4: bool = ((s_57_3.value()) != 0);
        // S s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // C s_57_6: const #0u : u8
        let s_57_6: bool = false;
        // C s_57_7: cast zx s_57_6 -> bv
        let s_57_7: Bits = Bits::new(s_57_6 as u128, 1u16);
        // S s_57_8: cmp-eq s_57_5 s_57_7
        let s_57_8: bool = ((s_57_5) == (s_57_7));
        // N s_57_9: branch s_57_8 b59 b58
        if s_57_8 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #102552u : u32
        let s_60_0: u32 = 102552;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_HCR_EL2_Type_RW(s_60_1)
        let s_60_2: bool = u_get_HCR_EL2_Type_RW(state, tracer, s_60_1);
        // D s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // C s_60_4: const #0u : u8
        let s_60_4: bool = false;
        // C s_60_5: cast zx s_60_4 -> bv
        let s_60_5: Bits = Bits::new(s_60_4 as u128, 1u16);
        // D s_60_6: cmp-eq s_60_3 s_60_5
        let s_60_6: bool = ((s_60_3) == (s_60_5));
        // D s_60_7: write-var gs#30965 <= s_60_6
        fn_state.gs_30965 = s_60_6;
        // N s_60_8: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #440u : u32
        let s_61_0: u32 = 440;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call ELUsingAArch32(s_61_1)
        let s_61_2: bool = ELUsingAArch32(state, tracer, s_61_1);
        // D s_61_3: not s_61_2
        let s_61_3: bool = !s_61_2;
        // D s_61_4: write-var gs#30945 <= s_61_3
        fn_state.gs_30945 = s_61_3;
        // N s_61_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_TGE(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_1);
        // D s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // C s_62_4: const #1u : u8
        let s_62_4: bool = true;
        // C s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 1u16);
        // D s_62_6: cmp-eq s_62_3 s_62_5
        let s_62_6: bool = ((s_62_3) == (s_62_5));
        // D s_62_7: write-var gs#30944 <= s_62_6
        fn_state.gs_30944 = s_62_6;
        // N s_62_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #432u : u32
        let s_63_0: u32 = 432;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call ELUsingAArch32(s_63_1)
        let s_63_2: bool = ELUsingAArch32(state, tracer, s_63_1);
        // D s_63_3: not s_63_2
        let s_63_3: bool = !s_63_2;
        // D s_63_4: write-var gs#30943 <= s_63_3
        fn_state.gs_30943 = s_63_3;
        // N s_63_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // D s_64_2: write-var gs#30942 <= s_64_1
        fn_state.gs_30942 = s_64_1;
        // N s_64_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call AArch64_CheckFPEnabled(s_65_0)
        let s_65_1: () = AArch64_CheckFPEnabled(state, tracer, s_65_0);
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call AArch64_CheckFPAdvSIMDEnabled(s_66_0)
        let s_66_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_66_0);
        // N s_66_2: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EL2Enabled(s_67_0)
        let s_67_1: bool = EL2Enabled(state, tracer, s_67_0);
        // S s_67_2: not s_67_1
        let s_67_2: bool = !s_67_1;
        // N s_67_3: branch s_67_2 b73 b68
        if s_67_2 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #432u : u32
        let s_68_0: u32 = 432;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call ELUsingAArch32(s_68_1)
        let s_68_2: bool = ELUsingAArch32(state, tracer, s_68_1);
        // D s_68_3: not s_68_2
        let s_68_3: bool = !s_68_2;
        // N s_68_4: branch s_68_3 b72 b69
        if s_68_3 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#30939 <= s_69_0
        fn_state.gs_30939 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#30939:u8
        let s_70_0: bool = fn_state.gs_30939;
        // D s_70_1: write-var gs#30940 <= s_70_0
        fn_state.gs_30940 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#30940:u8
        let s_71_0: bool = fn_state.gs_30940;
        // D s_71_1: write-var gs#30941 <= s_71_0
        fn_state.gs_30941 = s_71_0;
        // N s_71_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #102552u : u32
        let s_72_0: u32 = 102552;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_HCR_EL2_Type_TGE(s_72_1)
        let s_72_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_72_1);
        // D s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // C s_72_4: const #0u : u8
        let s_72_4: bool = false;
        // C s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 1u16);
        // D s_72_6: cmp-eq s_72_3 s_72_5
        let s_72_6: bool = ((s_72_3) == (s_72_5));
        // D s_72_7: write-var gs#30939 <= s_72_6
        fn_state.gs_30939 = s_72_6;
        // N s_72_8: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#30940 <= s_73_0
        fn_state.gs_30940 = s_73_0;
        // N s_73_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #440u : u32
        let s_74_0: u32 = 440;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // D s_74_4: write-var gs#30938 <= s_74_3
        fn_state.gs_30938 = s_74_3;
        // N s_74_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
