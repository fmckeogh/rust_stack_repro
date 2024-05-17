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
use u_get_MPAM3_EL3_Type_ALTSP_HFC::*;
use MPAM3_EL3_read::*;
use u_get_MPAM3_EL3_Type_ALTSP_HEN::*;
use UsePrimarySpaceEL10::*;
use EL2Enabled::*;
use UsePrimarySpaceEL2::*;
use ELIsInHost::*;
use Unreachable::*;
use common::*;
pub fn AltPIdSecure<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    primaryPIdSpace: u32,
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_6941: bool,
        PIdSpace: u32,
        el2en: bool,
        gs_6944: bool,
        el: u8,
        primaryPIdSpace: u32,
    }
    let fn_state = FunctionState {
        el,
        primaryPIdSpace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_0_0: read-var primaryPIdSpace:u32
        let s_0_0: u32 = fn_state.primaryPIdSpace;
        // D s_0_1: write-var PIdSpace <= s_0_0
        fn_state.PIdSpace = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call EL2Enabled(s_0_2)
        let s_0_3: bool = EL2Enabled(state, tracer, s_0_2);
        // D s_0_4: write-var el2en <= s_0_3
        fn_state.el2en = s_0_3;
        // D s_0_5: read-var el:u8
        let s_0_5: u8 = fn_state.el;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // C s_0_7: const #448u : u32
        let s_0_7: u32 = 448;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // D s_0_10: cmp-eq s_0_6 s_0_9
        let s_0_10: bool = ((s_0_6) == (s_0_9));
        // D s_0_11: not s_0_10
        let s_0_11: bool = !s_0_10;
        // N s_0_12: branch s_0_11 b17 b1
        if s_0_11 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var el2en:u8
        let s_1_0: bool = fn_state.el2en;
        // N s_1_1: branch s_1_0 b9 b2
        if s_1_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call MPAM3_EL3_read(s_2_0)
        let s_2_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_2_0);
        // S s_2_2: call _get_MPAM3_EL3_Type_ALTSP_HEN(s_2_1)
        let s_2_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HEN(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // S s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b8 b3
        if s_2_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#6941 <= s_3_0
        fn_state.gs_6941 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_4_0: read-var gs#6941:u8
        let s_4_0: bool = fn_state.gs_6941;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var PIdSpace:u32
        let s_6_0: u32 = fn_state.PIdSpace;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: write-var PIdSpace <= s_7_0
        fn_state.PIdSpace = s_7_0;
        // N s_7_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call MPAM3_EL3_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_8_0);
        // S s_8_2: call _get_MPAM3_EL3_Type_ALTSP_HFC(s_8_1)
        let s_8_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HFC(state, tracer, s_8_1);
        // S s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // S s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // D s_8_7: write-var gs#6941 <= s_8_6
        fn_state.gs_6941 = s_8_6;
        // N s_8_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #448u : u32
        let s_9_0: u32 = 448;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call ELIsInHost(s_9_1)
        let s_9_2: bool = ELIsInHost(state, tracer, s_9_1);
        // N s_9_3: branch s_9_2 b13 b10
        if s_9_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call UsePrimarySpaceEL10(s_10_0)
        let s_10_1: bool = UsePrimarySpaceEL10(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b12 b11
        if s_10_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_11_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #3u : u32
        let s_12_0: u32 = 3;
        // D s_12_1: write-var PIdSpace <= s_12_0
        fn_state.PIdSpace = s_12_0;
        // N s_12_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call UsePrimarySpaceEL2(s_13_0)
        let s_13_1: bool = UsePrimarySpaceEL2(state, tracer, s_13_0);
        // S s_13_2: not s_13_1
        let s_13_2: bool = !s_13_1;
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_15_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #3u : u32
        let s_16_0: u32 = 3;
        // D s_16_1: write-var PIdSpace <= s_16_0
        fn_state.PIdSpace = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var el:u8
        let s_17_0: u8 = fn_state.el;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #440u : u32
        let s_17_2: u32 = 440;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 2u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // D s_17_6: not s_17_5
        let s_17_6: bool = !s_17_5;
        // N s_17_7: branch s_17_6 b29 b18
        if s_17_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_18_0: read-var el2en:u8
        let s_18_0: bool = fn_state.el2en;
        // N s_18_1: branch s_18_0 b25 b19
        if s_18_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call MPAM3_EL3_read(s_19_0)
        let s_19_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_19_0);
        // S s_19_2: call _get_MPAM3_EL3_Type_ALTSP_HEN(s_19_1)
        let s_19_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HEN(state, tracer, s_19_1);
        // S s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // S s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // N s_19_7: branch s_19_6 b24 b20
        if s_19_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#6944 <= s_20_0
        fn_state.gs_6944 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var gs#6944:u8
        let s_21_0: bool = fn_state.gs_6944;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_22_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_23_0: const #3u : u32
        let s_23_0: u32 = 3;
        // D s_23_1: write-var PIdSpace <= s_23_0
        fn_state.PIdSpace = s_23_0;
        // N s_23_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call MPAM3_EL3_read(s_24_0)
        let s_24_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_24_0);
        // S s_24_2: call _get_MPAM3_EL3_Type_ALTSP_HFC(s_24_1)
        let s_24_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HFC(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#6944 <= s_24_6
        fn_state.gs_6944 = s_24_6;
        // N s_24_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call UsePrimarySpaceEL10(s_25_0)
        let s_25_1: bool = UsePrimarySpaceEL10(state, tracer, s_25_0);
        // S s_25_2: not s_25_1
        let s_25_2: bool = !s_25_1;
        // N s_25_3: branch s_25_2 b28 b26
        if s_25_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_26_0: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_27_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_28_0: const #3u : u32
        let s_28_0: u32 = 3;
        // D s_28_1: write-var PIdSpace <= s_28_0
        fn_state.PIdSpace = s_28_0;
        // N s_28_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_29_0: read-var el:u8
        let s_29_0: u8 = fn_state.el;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 2u16);
        // C s_29_2: const #432u : u32
        let s_29_2: u32 = 432;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: cast zx s_29_3 -> bv
        let s_29_4: Bits = Bits::new(s_29_3 as u128, 2u16);
        // D s_29_5: cmp-eq s_29_1 s_29_4
        let s_29_5: bool = ((s_29_1) == (s_29_4));
        // D s_29_6: not s_29_5
        let s_29_6: bool = !s_29_5;
        // N s_29_7: branch s_29_6 b34 b30
        if s_29_6 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call UsePrimarySpaceEL2(s_30_0)
        let s_30_1: bool = UsePrimarySpaceEL2(state, tracer, s_30_0);
        // S s_30_2: not s_30_1
        let s_30_2: bool = !s_30_1;
        // N s_30_3: branch s_30_2 b33 b31
        if s_30_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_31_0: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_32_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_33_0: const #3u : u32
        let s_33_0: u32 = 3;
        // D s_33_1: write-var PIdSpace <= s_33_0
        fn_state.PIdSpace = s_33_0;
        // N s_33_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call Unreachable(s_34_0)
        let s_34_1: () = Unreachable(state, tracer, s_34_0);
        // N s_34_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
