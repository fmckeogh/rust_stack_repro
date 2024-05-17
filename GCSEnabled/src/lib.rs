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
use u_get_HCR_EL2_Type_E2H::*;
use IsHCRXEL2Enabled::*;
use u_get_HCRX_EL2_Type_GCSEn::*;
use GCSPCRSelected::*;
use u_get_SCR_EL3_Type_GCSEn::*;
use UsingAArch32::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn GCSEnabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6791: bool,
        gs_6789: bool,
        gs_6785: bool,
        return_value: bool,
        gs_6784: bool,
        gs_6788: bool,
        gs_6786: bool,
        gs_6787: bool,
        gs_6790: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
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
        // N s_0_2: branch s_0_1 b31 b1
        if s_0_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // N s_1_4: branch s_1_3 b30 b2
        if s_1_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#6784 <= s_2_0
        fn_state.gs_6784 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#6784:u8
        let s_3_0: bool = fn_state.gs_6784;
        // N s_3_1: branch s_3_0 b29 b4
        if s_3_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#6785 <= s_4_0
        fn_state.gs_6785 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#6785:u8
        let s_5_0: bool = fn_state.gs_6785;
        // N s_5_1: branch s_5_0 b28 b6
        if s_5_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var el:u8
        let s_6_0: u8 = fn_state.el;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #448u : u32
        let s_6_2: u32 = 448;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b27 b7
        if s_6_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var el:u8
        let s_7_0: u8 = fn_state.el;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #440u : u32
        let s_7_2: u32 = 440;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: write-var gs#6786 <= s_7_5
        fn_state.gs_6786 = s_7_5;
        // N s_7_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var gs#6786:u8
        let s_8_0: bool = fn_state.gs_6786;
        // N s_8_1: branch s_8_0 b26 b9
        if s_8_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#6787 <= s_9_0
        fn_state.gs_6787 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#6787:u8
        let s_10_0: bool = fn_state.gs_6787;
        // N s_10_1: branch s_10_0 b22 b11
        if s_10_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#6789 <= s_11_0
        fn_state.gs_6789 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var gs#6789:u8
        let s_12_0: bool = fn_state.gs_6789;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#6791 <= s_13_0
        fn_state.gs_6791 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#6791:u8
        let s_14_0: bool = fn_state.gs_6791;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var el:u8
        let s_15_0: u8 = fn_state.el;
        // D s_15_1: call GCSPCRSelected(s_15_0)
        let s_15_1: bool = GCSPCRSelected(state, tracer, s_15_0);
        // D s_15_2: write-var return_value <= s_15_1
        fn_state.return_value = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var return_value:u8
        let s_16_0: bool = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var return_value <= s_17_0
        fn_state.return_value = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call IsHCRXEL2Enabled(s_18_0)
        let s_18_1: bool = IsHCRXEL2Enabled(state, tracer, s_18_0);
        // S s_18_2: not s_18_1
        let s_18_2: bool = !s_18_1;
        // N s_18_3: branch s_18_2 b21 b19
        if s_18_2 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #22528u : u32
        let s_19_0: u32 = 22528;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_HCRX_EL2_Type_GCSEn(s_19_1)
        let s_19_2: bool = u_get_HCRX_EL2_Type_GCSEn(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#6790 <= s_19_6
        fn_state.gs_6790 = s_19_6;
        // N s_19_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#6790:u8
        let s_20_0: bool = fn_state.gs_6790;
        // D s_20_1: write-var gs#6791 <= s_20_0
        fn_state.gs_6791 = s_20_0;
        // N s_20_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#6790 <= s_21_0
        fn_state.gs_6790 = s_21_0;
        // N s_21_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #102552u : u32
        let s_22_0: u32 = 102552;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_HCR_EL2_Type_TGE(s_22_1)
        let s_22_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #0u : u8
        let s_22_4: bool = false;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // N s_22_7: branch s_22_6 b25 b23
        if s_22_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #102552u : u32
        let s_23_0: u32 = 102552;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_HCR_EL2_Type_E2H(s_23_1)
        let s_23_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #0u : u8
        let s_23_4: bool = false;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#6788 <= s_23_6
        fn_state.gs_6788 = s_23_6;
        // N s_23_8: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#6788:u8
        let s_24_0: bool = fn_state.gs_6788;
        // D s_24_1: write-var gs#6789 <= s_24_0
        fn_state.gs_6789 = s_24_0;
        // N s_24_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#6788 <= s_25_0
        fn_state.gs_6788 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL2Enabled(s_26_0)
        let s_26_1: bool = EL2Enabled(state, tracer, s_26_0);
        // D s_26_2: write-var gs#6787 <= s_26_1
        fn_state.gs_6787 = s_26_1;
        // N s_26_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#6786 <= s_27_0
        fn_state.gs_6786 = s_27_0;
        // N s_27_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #90704u : u32
        let s_29_0: u32 = 90704;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_SCR_EL3_Type_GCSEn(s_29_1)
        let s_29_2: bool = u_get_SCR_EL3_Type_GCSEn(state, tracer, s_29_1);
        // D s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #0u : u8
        let s_29_4: bool = false;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // D s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // D s_29_7: write-var gs#6785 <= s_29_6
        fn_state.gs_6785 = s_29_6;
        // N s_29_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var el:u8
        let s_30_0: u8 = fn_state.el;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 2u16);
        // C s_30_2: const #424u : u32
        let s_30_2: u32 = 424;
        // D s_30_3: read-reg s_30_2:u8
        let s_30_3: u8 = {
            let value = state.read_register::<u8>(s_30_2 as isize);
            tracer.read_register(s_30_2 as isize, value);
            value
        };
        // D s_30_4: cast zx s_30_3 -> bv
        let s_30_4: Bits = Bits::new(s_30_3 as u128, 2u16);
        // D s_30_5: cmp-ne s_30_1 s_30_4
        let s_30_5: bool = ((s_30_1) != (s_30_4));
        // D s_30_6: write-var gs#6784 <= s_30_5
        fn_state.gs_6784 = s_30_5;
        // N s_30_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b16
        return block_16(state, tracer, fn_state);
    }
}
