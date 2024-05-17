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
use u_get_HCR_EL2_Type_TGE::*;
use u_get_MDCR_EL2_Type_TDE::*;
use Halted::*;
use ELUsingAArch32::*;
use u_get_OSLSR_EL1_Type_OSLK::*;
use IsSecureEL2Enabled::*;
use DoubleLockStatus::*;
use u_get_MDSCR_EL1_Type_KDE::*;
use u_get_SDER32_EL3_Type_SUIDEN::*;
use SDER32_EL3_read::*;
use u_get_MDCR_EL3_Type_SDD::*;
use common::*;
pub fn AArch64_GenerateDebugExceptionsFrom<T: Tracer>(
    state: &mut State,
    tracer: &T,
    from_el: u8,
    from_state: u32,
    mask: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_4892: bool,
        target: u8,
        gs_4890: bool,
        gs_4887: bool,
        gs_4900: bool,
        gs_4894: bool,
        gs_4897: bool,
        return_value: bool,
        gs_4898: bool,
        enabled: bool,
        gs_4893: bool,
        gs_4888: bool,
        gs_4902: bool,
        gs_4896: bool,
        gs_4891: bool,
        from_el: u8,
        from_state: u32,
        mask: bool,
    }
    let fn_state = FunctionState {
        from_el,
        from_state,
        mask,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #10128u : u32
        let s_0_0: u32 = 10128;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_OSLSR_EL1_Type_OSLK(s_0_1)
        let s_0_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b51 b1
        if s_0_6 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call DoubleLockStatus(s_1_0)
        let s_1_1: bool = DoubleLockStatus(state, tracer, s_1_0);
        // D s_1_2: write-var gs#4887 <= s_1_1
        fn_state.gs_4887 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#4887:u8
        let s_2_0: bool = fn_state.gs_4887;
        // N s_2_1: branch s_2_0 b50 b3
        if s_2_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call Halted(s_3_0)
        let s_3_1: bool = Halted(state, tracer, s_3_0);
        // D s_3_2: write-var gs#4888 <= s_3_1
        fn_state.gs_4888 = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#4888:u8
        let s_4_0: bool = fn_state.gs_4888;
        // N s_4_1: branch s_4_0 b49 b5
        if s_4_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // N s_5_4: branch s_5_3 b45 b6
        if s_5_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#4891 <= s_6_0
        fn_state.gs_4891 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#4891:u8
        let s_7_0: bool = fn_state.gs_4891;
        // N s_7_1: branch s_7_0 b41 b8
        if s_7_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#4893 <= s_8_0
        fn_state.gs_4893 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#4893:u8
        let s_9_0: bool = fn_state.gs_4893;
        // N s_9_1: branch s_9_0 b40 b10
        if s_9_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #440u : u32
        let s_10_0: u32 = 440;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: write-var target <= s_10_1
        fn_state.target = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #424u : u32
        let s_11_0: u32 = 424;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // D s_11_3: cmp-lt s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) < (s_11_2));
        // N s_11_4: branch s_11_3 b39 b12
        if s_11_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#4894 <= s_12_0
        fn_state.gs_4894 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#4894:u8
        let s_13_0: bool = fn_state.gs_4894;
        // N s_13_1: branch s_13_0 b29 b14
        if s_13_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var enabled <= s_14_0
        fn_state.enabled = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var from_el:u8
        let s_15_0: u8 = fn_state.from_el;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 2u16);
        // D s_15_2: read-var target:u8
        let s_15_2: u8 = fn_state.target;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b22 b16
        if s_15_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var enabled:u8
        let s_16_0: bool = fn_state.enabled;
        // N s_16_1: branch s_16_0 b21 b17
        if s_16_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#4896 <= s_17_0
        fn_state.gs_4896 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#4896:u8
        let s_18_0: bool = fn_state.gs_4896;
        // D s_18_1: write-var enabled <= s_18_0
        fn_state.enabled = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var enabled:u8
        let s_19_0: bool = fn_state.enabled;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var return_value:u8
        let s_20_0: bool = fn_state.return_value;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var target:u8
        let s_21_0: u8 = fn_state.target;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // D s_21_4: read-var from_el:u8
        let s_21_4: u8 = fn_state.from_el;
        // D s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 2u16);
        // D s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (s_21_5.value() as i128);
        // D s_21_7: cast reint s_21_6 -> i64
        let s_21_7: i64 = (s_21_6 as i64);
        // D s_21_8: cast zx s_21_3 -> i
        let s_21_8: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_9: cast zx s_21_7 -> i
        let s_21_9: i128 = (i128::try_from(s_21_7).unwrap());
        // D s_21_10: cmp-gt s_21_8 s_21_9
        let s_21_10: bool = ((s_21_8) > (s_21_9));
        // D s_21_11: write-var gs#4896 <= s_21_10
        fn_state.gs_4896 = s_21_10;
        // N s_21_12: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var enabled:u8
        let s_22_0: bool = fn_state.enabled;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#4897 <= s_23_0
        fn_state.gs_4897 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#4897:u8
        let s_24_0: bool = fn_state.gs_4897;
        // N s_24_1: branch s_24_0 b27 b25
        if s_24_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#4898 <= s_25_0
        fn_state.gs_4898 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#4898:u8
        let s_26_0: bool = fn_state.gs_4898;
        // D s_26_1: write-var enabled <= s_26_0
        fn_state.enabled = s_26_0;
        // N s_26_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var mask:u8
        let s_27_0: bool = fn_state.mask;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#4898 <= s_27_4
        fn_state.gs_4898 = s_27_4;
        // N s_27_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #104648u : u32
        let s_28_0: u32 = 104648;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_MDSCR_EL1_Type_KDE(s_28_1)
        let s_28_2: bool = u_get_MDSCR_EL1_Type_KDE(state, tracer, s_28_1);
        // D s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // D s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#4897 <= s_28_6
        fn_state.gs_4897 = s_28_6;
        // N s_28_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #22712u : u32
        let s_29_0: u32 = 22712;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_MDCR_EL3_Type_SDD(s_29_1)
        let s_29_2: bool = u_get_MDCR_EL3_Type_SDD(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var enabled <= s_29_6
        fn_state.enabled = s_29_6;
        // D s_29_8: read-var from_el:u8
        let s_29_8: u8 = fn_state.from_el;
        // D s_29_9: cast zx s_29_8 -> bv
        let s_29_9: Bits = Bits::new(s_29_8 as u128, 2u16);
        // C s_29_10: const #448u : u32
        let s_29_10: u32 = 448;
        // D s_29_11: read-reg s_29_10:u8
        let s_29_11: u8 = {
            let value = state.read_register::<u8>(s_29_10 as isize);
            tracer.read_register(s_29_10 as isize, value);
            value
        };
        // D s_29_12: cast zx s_29_11 -> bv
        let s_29_12: Bits = Bits::new(s_29_11 as u128, 2u16);
        // D s_29_13: cmp-eq s_29_9 s_29_12
        let s_29_13: bool = ((s_29_9) == (s_29_12));
        // N s_29_14: branch s_29_13 b38 b30
        if s_29_13 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#4900 <= s_30_0
        fn_state.gs_4900 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_31_0: read-var gs#4900:u8
        let s_31_0: bool = fn_state.gs_4900;
        // N s_31_1: branch s_31_0 b34 b32
        if s_31_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_32_0: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_33_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var enabled:u8
        let s_34_0: bool = fn_state.enabled;
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
    ) -> bool {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call SDER32_EL3_read(s_35_0)
        let s_35_1: ProductType5c790c8ef59cc8b2 = SDER32_EL3_read(state, tracer, s_35_0);
        // S s_35_2: call _get_SDER32_EL3_Type_SUIDEN(s_35_1)
        let s_35_2: bool = u_get_SDER32_EL3_Type_SUIDEN(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#4902 <= s_35_6
        fn_state.gs_4902 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#4902:u8
        let s_36_0: bool = fn_state.gs_4902;
        // D s_36_1: write-var enabled <= s_36_0
        fn_state.enabled = s_36_0;
        // N s_36_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#4902 <= s_37_0
        fn_state.gs_4902 = s_37_0;
        // N s_37_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #440u : u32
        let s_38_0: u32 = 440;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: write-var gs#4900 <= s_38_2
        fn_state.gs_4900 = s_38_2;
        // N s_38_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_39_0: read-var from_state:u32
        let s_39_0: u32 = fn_state.from_state;
        // C s_39_1: const #3u : u32
        let s_39_1: u32 = 3;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: write-var gs#4894 <= s_39_2
        fn_state.gs_4894 = s_39_2;
        // N s_39_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #432u : u32
        let s_40_0: u32 = 432;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: write-var target <= s_40_1
        fn_state.target = s_40_1;
        // N s_40_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #102552u : u32
        let s_41_0: u32 = 102552;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_HCR_EL2_Type_TGE(s_41_1)
        let s_41_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_41_1);
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #1u : u8
        let s_41_4: bool = true;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // D s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // N s_41_7: branch s_41_6 b44 b42
        if s_41_6 {
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
        // C s_42_0: const #104880u : u32
        let s_42_0: u32 = 104880;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_MDCR_EL2_Type_TDE(s_42_1)
        let s_42_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #1u : u8
        let s_42_4: bool = true;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#4892 <= s_42_6
        fn_state.gs_4892 = s_42_6;
        // N s_42_8: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var gs#4892:u8
        let s_43_0: bool = fn_state.gs_4892;
        // D s_43_1: write-var gs#4893 <= s_43_0
        fn_state.gs_4893 = s_43_0;
        // N s_43_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#4892 <= s_44_0
        fn_state.gs_4892 = s_44_0;
        // N s_44_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var from_state:u32
        let s_45_0: u32 = fn_state.from_state;
        // C s_45_1: const #3u : u32
        let s_45_1: u32 = 3;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // N s_45_3: branch s_45_2 b48 b46
        if s_45_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call IsSecureEL2Enabled(s_46_0)
        let s_46_1: bool = IsSecureEL2Enabled(state, tracer, s_46_0);
        // D s_46_2: write-var gs#4890 <= s_46_1
        fn_state.gs_4890 = s_46_1;
        // N s_46_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var gs#4890:u8
        let s_47_0: bool = fn_state.gs_4890;
        // D s_47_1: write-var gs#4891 <= s_47_0
        fn_state.gs_4891 = s_47_0;
        // N s_47_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#4890 <= s_48_0
        fn_state.gs_4890 = s_48_0;
        // N s_48_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var return_value <= s_49_0
        fn_state.return_value = s_49_0;
        // N s_49_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#4888 <= s_50_0
        fn_state.gs_4888 = s_50_0;
        // N s_50_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#4887 <= s_51_0
        fn_state.gs_4887 = s_51_0;
        // N s_51_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
