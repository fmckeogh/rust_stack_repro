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
use AArch64_S1E0POEnabled::*;
use AArch64_S1OverlayPermissions::*;
use AArch64_S1DirectBasePermissions::*;
use AArch64_S1POEnabled::*;
use AArch64_S1IndirectBasePermissions::*;
use common::*;
pub fn AArch64_S1ComputePermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeef284266e139aee2,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypea231b9ca5c98dc3c {
    #[derive(Default)]
    struct FunctionState {
        gs_17335: bool,
        s1perms: ProductTypea231b9ca5c98dc3c,
        gs_17334: bool,
        s1overlay_permsshadow_296: ProductTypea231b9ca5c98dc3c,
        gs_17333: bool,
        gs_17341: bool,
        regime: u32,
        walkstate: ProductType96e7acababe246a1,
        walkparams: ProductTypeef284266e139aee2,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        regime,
        walkstate,
        walkparams,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_0_0: read-var walkparams.24:struct
        let s_0_0: bool = fn_state.walkparams._24;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b28 b1
        if s_0_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // D s_1_1: read-var walkstate:struct
        let s_1_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_1_2: read-var walkparams:struct
        let s_1_2: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_1_3: read-var accdesc:struct
        let s_1_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_4: call AArch64_S1DirectBasePermissions(s_1_0, s_1_1, s_1_2, s_1_3)
        let s_1_4: ProductTypea231b9ca5c98dc3c = AArch64_S1DirectBasePermissions(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
            s_1_3,
        );
        // D s_1_5: write-var s1perms <= s_1_4
        fn_state.s1perms = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_2_0: read-var accdesc.8:struct
        let s_2_0: u8 = fn_state.accdesc._8;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #448u : u32
        let s_2_2: u32 = 448;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b27 b3
        if s_2_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#17333 <= s_3_0
        fn_state.gs_17333 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_4_0: read-var gs#17333:u8
        let s_4_0: bool = fn_state.gs_17333;
        // N s_4_1: branch s_4_0 b26 b5
        if s_4_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_5_0: read-var accdesc.8:struct
        let s_5_0: u8 = fn_state.accdesc._8;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #448u : u32
        let s_5_2: u32 = 448;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-ne s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) != (s_5_4));
        // N s_5_6: branch s_5_5 b25 b6
        if s_5_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#17341 <= s_6_0
        fn_state.gs_17341 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_7_0: read-var gs#17341:u8
        let s_7_0: bool = fn_state.gs_17341;
        // N s_7_1: branch s_7_0 b24 b8
        if s_7_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_9_0: read-var s1perms.2:struct
        let s_9_0: bool = fn_state.s1perms._2;
        // N s_9_1: branch s_9_0 b23 b10
        if s_9_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_11_0: read-var s1perms.2:struct
        let s_11_0: bool = fn_state.s1perms._2;
        // N s_11_1: branch s_11_0 b22 b12
        if s_11_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#17334 <= s_12_0
        fn_state.gs_17334 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_13_0: read-var gs#17334:u8
        let s_13_0: bool = fn_state.gs_17334;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#17335 <= s_14_0
        fn_state.gs_17335 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_15_0: read-var gs#17335:u8
        let s_15_0: bool = fn_state.gs_17335;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_16_0: read-var s1perms.7:struct
        let s_16_0: bool = fn_state.s1perms._7;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b19 b17
        if s_16_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_18_0: read-var s1perms:struct
        let s_18_0: ProductTypea231b9ca5c98dc3c = fn_state.s1perms;
        // N s_18_1: return s_18_0
        return s_18_0;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var s1perms.8 <= s_19_0
        fn_state.s1perms._8 = s_19_0;
        // N s_19_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var s1perms.3 <= s_20_0
        fn_state.s1perms._3 = s_20_0;
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_21_0: read-var s1perms.4:struct
        let s_21_0: bool = fn_state.s1perms._4;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#17335 <= s_21_4
        fn_state.gs_17335 = s_21_4;
        // N s_21_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_22_0: read-var s1perms.7:struct
        let s_22_0: bool = fn_state.s1perms._7;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#17334 <= s_22_4
        fn_state.gs_17334 = s_22_4;
        // N s_22_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_23_0: read-var regime:u32
        let s_23_0: u32 = fn_state.regime;
        // D s_23_1: read-var walkstate:struct
        let s_23_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_23_2: read-var accdesc:struct
        let s_23_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_23_3: call AArch64_S1OverlayPermissions(s_23_0, s_23_1, s_23_2)
        let s_23_3: ProductTypea231b9ca5c98dc3c = AArch64_S1OverlayPermissions(
            state,
            tracer,
            s_23_0,
            s_23_1,
            s_23_2,
        );
        // D s_23_4: write-var s1overlay_permsshadow#296 <= s_23_3
        fn_state.s1overlay_permsshadow_296 = s_23_3;
        // D s_23_5: read-var s1overlay_permsshadow#296.1:struct
        let s_23_5: bool = fn_state.s1overlay_permsshadow_296._1;
        // D s_23_6: write-var s1perms.1 <= s_23_5
        fn_state.s1perms._1 = s_23_5;
        // D s_23_7: read-var s1overlay_permsshadow#296.3:struct
        let s_23_7: bool = fn_state.s1overlay_permsshadow_296._3;
        // D s_23_8: write-var s1perms.3 <= s_23_7
        fn_state.s1perms._3 = s_23_7;
        // D s_23_9: read-var s1overlay_permsshadow#296.4:struct
        let s_23_9: bool = fn_state.s1overlay_permsshadow_296._4;
        // D s_23_10: write-var s1perms.4 <= s_23_9
        fn_state.s1perms._4 = s_23_9;
        // N s_23_11: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var s1perms.2 <= s_24_0
        fn_state.s1perms._2 = s_24_0;
        // N s_24_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_25_0: read-var regime:u32
        let s_25_0: u32 = fn_state.regime;
        // D s_25_1: call AArch64_S1POEnabled(s_25_0)
        let s_25_1: bool = AArch64_S1POEnabled(state, tracer, s_25_0);
        // D s_25_2: not s_25_1
        let s_25_2: bool = !s_25_1;
        // D s_25_3: write-var gs#17341 <= s_25_2
        fn_state.gs_17341 = s_25_2;
        // N s_25_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var s1perms.2 <= s_26_0
        fn_state.s1perms._2 = s_26_0;
        // N s_26_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_27_0: read-var walkparams.22:struct
        let s_27_0: bool = fn_state.walkparams._22;
        // D s_27_1: read-var regime:u32
        let s_27_1: u32 = fn_state.regime;
        // D s_27_2: call AArch64_S1E0POEnabled(s_27_1, s_27_0)
        let s_27_2: bool = AArch64_S1E0POEnabled(state, tracer, s_27_1, s_27_0);
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // D s_27_4: write-var gs#17333 <= s_27_3
        fn_state.gs_17333 = s_27_3;
        // N s_27_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypea231b9ca5c98dc3c {
        // D s_28_0: read-var regime:u32
        let s_28_0: u32 = fn_state.regime;
        // D s_28_1: read-var walkstate:struct
        let s_28_1: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_28_2: read-var walkparams:struct
        let s_28_2: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_28_3: read-var accdesc:struct
        let s_28_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_28_4: call AArch64_S1IndirectBasePermissions(s_28_0, s_28_1, s_28_2, s_28_3)
        let s_28_4: ProductTypea231b9ca5c98dc3c = AArch64_S1IndirectBasePermissions(
            state,
            tracer,
            s_28_0,
            s_28_1,
            s_28_2,
            s_28_3,
        );
        // D s_28_5: write-var s1perms <= s_28_4
        fn_state.s1perms = s_28_4;
        // N s_28_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
