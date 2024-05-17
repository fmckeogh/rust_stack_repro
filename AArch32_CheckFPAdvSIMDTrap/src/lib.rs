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
use EL3SDDUndefPriority::*;
use AArch64_CheckFPAdvSIMDTrap::*;
use u_get_NSACR_Type_NSASEDIS::*;
use CurrentSecurityState::*;
use ConditionSyndrome::*;
use u_get_CPTR_EL3_Type_TFP::*;
use AArch64_AdvSIMDFPAccessTrap::*;
use ELUsingAArch32::*;
use HCPTR_read::*;
use ExceptionSyndrome::*;
use AArch32_TakeHypTrapException__1::*;
use Bit::*;
use u_get_NSACR_Type_cp10::*;
use AArch32_TakeUndefInstrException__1::*;
use EL2Enabled::*;
use u_get_HCPTR_Type_TCP10::*;
use EL3SDDUndef::*;
use u_get_HCPTR_Type_TASE::*;
use common::*;
pub fn AArch32_CheckFPAdvSIMDTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30910: bool,
        gs_30917: bool,
        gs_30907: bool,
        gs_30906: bool,
        except: ProductTypeb7f99f96751e17c4,
        hcptr_cp10: bool,
        gs_30911: bool,
        hcptr_tase: bool,
        gs_30908: bool,
        ssshadow_568: u32,
        gs_30915: bool,
        gs_30905: bool,
        gs_30916: bool,
        advsimd: bool,
    }
    let fn_state = FunctionState {
        advsimd,
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
        // S s_0_1: call EL2Enabled(s_0_0)
        let s_0_1: bool = EL2Enabled(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b57 b1
        if s_0_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#30905 <= s_1_0
        fn_state.gs_30905 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30905:u8
        let s_2_0: bool = fn_state.gs_30905;
        // N s_2_1: branch s_2_0 b56 b3
        if s_2_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // N s_3_4: branch s_3_3 b55 b4
        if s_3_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#30906 <= s_4_0
        fn_state.gs_30906 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#30906:u8
        let s_5_0: bool = fn_state.gs_30906;
        // N s_5_1: branch s_5_0 b54 b6
        if s_5_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#30907 <= s_6_0
        fn_state.gs_30907 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#30907:u8
        let s_7_0: bool = fn_state.gs_30907;
        // N s_7_1: branch s_7_0 b53 b8
        if s_7_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#30908 <= s_8_0
        fn_state.gs_30908 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#30908:u8
        let s_9_0: bool = fn_state.gs_30908;
        // N s_9_1: branch s_9_0 b52 b10
        if s_9_0 {
            return block_52(state, tracer, fn_state);
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
        // S s_10_1: call CurrentSecurityState(s_10_0)
        let s_10_1: u32 = CurrentSecurityState(state, tracer, s_10_0);
        // D s_10_2: write-var ssshadow#568 <= s_10_1
        fn_state.ssshadow_568 = s_10_1;
        // C s_10_3: const #432u : u32
        let s_10_3: u32 = 432;
        // D s_10_4: read-reg s_10_3:u8
        let s_10_4: u8 = {
            let value = state.read_register::<u8>(s_10_3 as isize);
            tracer.read_register(s_10_3 as isize, value);
            value
        };
        // C s_10_5: const #2u : u8
        let s_10_5: u8 = 2;
        // D s_10_6: cmp-lt s_10_4 s_10_5
        let s_10_6: bool = ((s_10_4) < (s_10_5));
        // N s_10_7: branch s_10_6 b51 b11
        if s_10_6 {
            return block_51(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#30910 <= s_11_0
        fn_state.gs_30910 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#30910:u8
        let s_12_0: bool = fn_state.gs_30910;
        // N s_12_1: branch s_12_0 b24 b13
        if s_12_0 {
            return block_24(state, tracer, fn_state);
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
        // C s_14_0: const #424u : u32
        let s_14_0: u32 = 424;
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
        // N s_14_4: branch s_14_3 b23 b15
        if s_14_3 {
            return block_23(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#30911 <= s_15_0
        fn_state.gs_30911 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#30911:u8
        let s_16_0: bool = fn_state.gs_30911;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #16840u : u32
        let s_18_0: u32 = 16840;
        // D s_18_1: read-reg s_18_0:struct
        let s_18_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call _get_CPTR_EL3_Type_TFP(s_18_1)
        let s_18_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_18_1);
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // C s_18_4: const #1u : u8
        let s_18_4: bool = true;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 1u16);
        // D s_18_6: cmp-eq s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) == (s_18_5));
        // N s_18_7: branch s_18_6 b20 b19
        if s_18_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL3SDDUndef(s_20_0)
        let s_20_1: bool = EL3SDDUndef(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b22 b21
        if s_20_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call AArch64_AdvSIMDFPAccessTrap(s_21_1)
        let s_21_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_21_1);
        // N s_21_3: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #424u : u32
        let s_23_0: u32 = 424;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call ELUsingAArch32(s_23_1)
        let s_23_2: bool = ELUsingAArch32(state, tracer, s_23_1);
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // D s_23_4: write-var gs#30911 <= s_23_3
        fn_state.gs_30911 = s_23_3;
        // N s_23_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HCPTR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_HCPTR_Type_TASE(s_24_1)
        let s_24_2: bool = u_get_HCPTR_Type_TASE(state, tracer, s_24_1);
        // D s_24_3: write-var hcptr_tase <= s_24_2
        fn_state.hcptr_tase = s_24_2;
        // C s_24_4: const #() : ()
        let s_24_4: () = ();
        // S s_24_5: call HCPTR_read(s_24_4)
        let s_24_5: ProductType700c18a878c5601b = HCPTR_read(state, tracer, s_24_4);
        // S s_24_6: call _get_HCPTR_Type_TCP10(s_24_5)
        let s_24_6: bool = u_get_HCPTR_Type_TCP10(state, tracer, s_24_5);
        // D s_24_7: write-var hcptr_cp10 <= s_24_6
        fn_state.hcptr_cp10 = s_24_6;
        // C s_24_8: const #424u : u32
        let s_24_8: u32 = 424;
        // D s_24_9: read-reg s_24_8:u8
        let s_24_9: u8 = {
            let value = state.read_register::<u8>(s_24_8 as isize);
            tracer.read_register(s_24_8 as isize, value);
            value
        };
        // C s_24_10: const #2u : u8
        let s_24_10: u8 = 2;
        // D s_24_11: cmp-lt s_24_9 s_24_10
        let s_24_11: bool = ((s_24_9) < (s_24_10));
        // N s_24_12: branch s_24_11 b50 b25
        if s_24_11 {
            return block_50(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#30915 <= s_25_0
        fn_state.gs_30915 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#30915:u8
        let s_26_0: bool = fn_state.gs_30915;
        // N s_26_1: branch s_26_0 b43 b27
        if s_26_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#30916 <= s_29_0
        fn_state.gs_30916 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#30916:u8
        let s_30_0: bool = fn_state.gs_30916;
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
        // D s_31_0: read-var hcptr_cp10:u8
        let s_31_0: bool = fn_state.hcptr_cp10;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#30917 <= s_31_4
        fn_state.gs_30917 = s_31_4;
        // N s_31_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#30917:u8
        let s_32_0: bool = fn_state.gs_30917;
        // N s_32_1: branch s_32_0 b35 b33
        if s_32_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #7u : u32
        let s_35_0: u32 = 7;
        // S s_35_1: call ExceptionSyndrome(s_35_0)
        let s_35_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_35_0,
        );
        // D s_35_2: write-var except <= s_35_1
        fn_state.except = s_35_1;
        // C s_35_3: const #() : ()
        let s_35_3: () = ();
        // S s_35_4: call ConditionSyndrome(s_35_3)
        let s_35_4: u8 = ConditionSyndrome(state, tracer, s_35_3);
        // D s_35_5: read-var except:struct
        let s_35_5: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_35_6: write-var except <= s_35_5
        fn_state.except = s_35_5;
        // D s_35_7: read-var advsimd:u8
        let s_35_7: bool = fn_state.advsimd;
        // N s_35_8: branch s_35_7 b40 b36
        if s_35_7 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // S s_36_1: call Bit(s_36_0)
        let s_36_1: bool = Bit(state, tracer, s_36_0);
        // D s_36_2: read-var except:struct
        let s_36_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_36_3: write-var except <= s_36_2
        fn_state.except = s_36_2;
        // D s_36_4: read-var except:struct
        let s_36_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_36_5: write-var except <= s_36_4
        fn_state.except = s_36_4;
        // N s_36_6: jump b37
        return block_37(state, tracer, fn_state);
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
        // C s_37_3: const #432u : u32
        let s_37_3: u32 = 432;
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
        // N s_37_7: branch s_37_6 b39 b38
        if s_37_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var except:struct
        let s_38_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_38_1: call AArch32_TakeHypTrapException__1(s_38_0)
        let s_38_1: () = AArch32_TakeHypTrapException__1(state, tracer, s_38_0);
        // N s_38_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var except:struct
        let s_39_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_39_1: call AArch32_TakeUndefInstrException__1(s_39_0)
        let s_39_1: () = AArch32_TakeUndefInstrException__1(state, tracer, s_39_0);
        // N s_39_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // S s_40_1: call Bit(s_40_0)
        let s_40_1: bool = Bit(state, tracer, s_40_0);
        // D s_40_2: read-var except:struct
        let s_40_2: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_40_3: write-var except <= s_40_2
        fn_state.except = s_40_2;
        // N s_40_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#30917 <= s_41_0
        fn_state.gs_30917 = s_41_0;
        // N s_41_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var hcptr_tase:u8
        let s_42_0: bool = fn_state.hcptr_tase;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#30916 <= s_42_4
        fn_state.gs_30916 = s_42_4;
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
        // N s_47_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var hcptr_cp10 <= s_48_0
        fn_state.hcptr_cp10 = s_48_0;
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
        // D s_49_1: write-var hcptr_tase <= s_49_0
        fn_state.hcptr_tase = s_49_0;
        // N s_49_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: write-var gs#30915 <= s_50_2
        fn_state.gs_30915 = s_50_2;
        // N s_50_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var ssshadow#568:u32
        let s_51_0: u32 = fn_state.ssshadow_568;
        // C s_51_1: const #3u : u32
        let s_51_1: u32 = 3;
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // D s_51_3: write-var gs#30910 <= s_51_2
        fn_state.gs_30910 = s_51_2;
        // N s_51_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL3SDDUndefPriority(s_53_0)
        let s_53_1: bool = EL3SDDUndefPriority(state, tracer, s_53_0);
        // D s_53_2: write-var gs#30908 <= s_53_1
        fn_state.gs_30908 = s_53_1;
        // N s_53_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #16840u : u32
        let s_54_0: u32 = 16840;
        // D s_54_1: read-reg s_54_0:struct
        let s_54_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // D s_54_2: call _get_CPTR_EL3_Type_TFP(s_54_1)
        let s_54_2: bool = u_get_CPTR_EL3_Type_TFP(state, tracer, s_54_1);
        // D s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // C s_54_4: const #1u : u8
        let s_54_4: bool = true;
        // C s_54_5: cast zx s_54_4 -> bv
        let s_54_5: Bits = Bits::new(s_54_4 as u128, 1u16);
        // D s_54_6: cmp-eq s_54_3 s_54_5
        let s_54_6: bool = ((s_54_3) == (s_54_5));
        // D s_54_7: write-var gs#30907 <= s_54_6
        fn_state.gs_30907 = s_54_6;
        // N s_54_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // D s_55_4: write-var gs#30906 <= s_55_3
        fn_state.gs_30906 = s_55_3;
        // N s_55_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call AArch64_CheckFPAdvSIMDTrap(s_56_0)
        let s_56_1: () = AArch64_CheckFPAdvSIMDTrap(state, tracer, s_56_0);
        // N s_56_2: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #432u : u32
        let s_57_0: u32 = 432;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#30905 <= s_57_3
        fn_state.gs_30905 = s_57_3;
        // N s_57_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
