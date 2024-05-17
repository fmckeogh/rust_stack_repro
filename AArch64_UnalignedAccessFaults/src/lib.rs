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
use AlignmentEnforced::*;
use u_get_SCTLRType_nAA::*;
use SCTLR_read__1::*;
use HaveLSE2Ext::*;
use AllInAlignedQuantity::*;
use common::*;
pub fn AArch64_UnalignedAccessFaults<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accdesc: ProductType9878976b5bcce9c9,
    address: u64,
    size: i128,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_15826: bool,
        gs_15823: bool,
        gs_15824: bool,
        gs_15822: bool,
        gs_15827: bool,
        return_value: bool,
        gs_15829: bool,
        accdesc: ProductType9878976b5bcce9c9,
        address: u64,
        size: i128,
    }
    let fn_state = FunctionState {
        accdesc,
        address,
        size,
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
        // S s_0_1: call AlignmentEnforced(s_0_0)
        let s_0_1: bool = AlignmentEnforced(state, tracer, s_0_0);
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
        // D s_1_0: read-var accdesc.1:struct
        let s_1_0: u32 = fn_state.accdesc._1;
        // C s_1_1: const #11u : u32
        let s_1_1: u32 = 11;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b30 b2
        if s_1_2 {
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
        // D s_2_0: read-var accdesc.21:struct
        let s_2_0: bool = fn_state.accdesc._21;
        // N s_2_1: branch s_2_0 b29 b3
        if s_2_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var accdesc.13:struct
        let s_3_0: bool = fn_state.accdesc._13;
        // N s_3_1: branch s_3_0 b28 b4
        if s_3_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var accdesc.9:struct
        let s_4_0: bool = fn_state.accdesc._9;
        // N s_4_1: branch s_4_0 b27 b5
        if s_4_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var accdesc.4:struct
        let s_5_0: bool = fn_state.accdesc._4;
        // D s_5_1: write-var gs#15822 <= s_5_0
        fn_state.gs_15822 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var gs#15822:u8
        let s_6_0: bool = fn_state.gs_15822;
        // N s_6_1: branch s_6_0 b23 b7
        if s_6_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var accdesc.3:struct
        let s_7_0: bool = fn_state.accdesc._3;
        // N s_7_1: branch s_7_0 b22 b8
        if s_7_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var accdesc.2:struct
        let s_8_0: bool = fn_state.accdesc._2;
        // D s_8_1: write-var gs#15823 <= s_8_0
        fn_state.gs_15823 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#15823:u8
        let s_9_0: bool = fn_state.gs_15823;
        // N s_9_1: branch s_9_0 b21 b10
        if s_9_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var accdesc.24:struct
        let s_10_0: bool = fn_state.accdesc._24;
        // D s_10_1: write-var gs#15824 <= s_10_0
        fn_state.gs_15824 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#15824:u8
        let s_11_0: bool = fn_state.gs_15824;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var return_value:u8
        let s_13_0: bool = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveLSE2Ext(s_14_0)
        let s_14_1: bool = HaveLSE2Ext(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // N s_14_3: branch s_14_2 b20 b15
        if s_14_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call SCTLR_read__1(s_15_0)
        let s_15_1: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_15_0);
        // S s_15_2: call _get_SCTLRType_nAA(s_15_1)
        let s_15_2: bool = u_get_SCTLRType_nAA(state, tracer, s_15_1);
        // S s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // C s_15_4: const #0u : u8
        let s_15_4: bool = false;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 1u16);
        // S s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // N s_15_7: branch s_15_6 b19 b16
        if s_15_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#15826 <= s_16_0
        fn_state.gs_15826 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var gs#15826:u8
        let s_17_0: bool = fn_state.gs_15826;
        // D s_17_1: write-var gs#15827 <= s_17_0
        fn_state.gs_15827 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#15827:u8
        let s_18_0: bool = fn_state.gs_15827;
        // D s_18_1: write-var return_value <= s_18_0
        fn_state.return_value = s_18_0;
        // N s_18_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #16s : i
        let s_19_0: i128 = 16;
        // D s_19_1: read-var address:u64
        let s_19_1: u64 = fn_state.address;
        // D s_19_2: read-var size:i
        let s_19_2: i128 = fn_state.size;
        // D s_19_3: call AllInAlignedQuantity(s_19_1, s_19_2, s_19_0)
        let s_19_3: bool = AllInAlignedQuantity(state, tracer, s_19_1, s_19_2, s_19_0);
        // D s_19_4: not s_19_3
        let s_19_4: bool = !s_19_3;
        // D s_19_5: write-var gs#15826 <= s_19_4
        fn_state.gs_15826 = s_19_4;
        // N s_19_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#15827 <= s_20_0
        fn_state.gs_15827 = s_20_0;
        // N s_20_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#15824 <= s_21_0
        fn_state.gs_15824 = s_21_0;
        // N s_21_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#15823 <= s_22_0
        fn_state.gs_15823 = s_22_0;
        // N s_22_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call HaveLSE2Ext(s_23_0)
        let s_23_1: bool = HaveLSE2Ext(state, tracer, s_23_0);
        // S s_23_2: not s_23_1
        let s_23_2: bool = !s_23_1;
        // N s_23_3: branch s_23_2 b26 b24
        if s_23_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #16s : i
        let s_24_0: i128 = 16;
        // D s_24_1: read-var address:u64
        let s_24_1: u64 = fn_state.address;
        // D s_24_2: read-var size:i
        let s_24_2: i128 = fn_state.size;
        // D s_24_3: call AllInAlignedQuantity(s_24_1, s_24_2, s_24_0)
        let s_24_3: bool = AllInAlignedQuantity(state, tracer, s_24_1, s_24_2, s_24_0);
        // D s_24_4: not s_24_3
        let s_24_4: bool = !s_24_3;
        // D s_24_5: write-var gs#15829 <= s_24_4
        fn_state.gs_15829 = s_24_4;
        // N s_24_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var gs#15829:u8
        let s_25_0: bool = fn_state.gs_15829;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#15829 <= s_26_0
        fn_state.gs_15829 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#15822 <= s_27_0
        fn_state.gs_15822 = s_27_0;
        // N s_27_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var return_value <= s_28_0
        fn_state.return_value = s_28_0;
        // N s_28_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var return_value <= s_29_0
        fn_state.return_value = s_29_0;
        // N s_29_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var return_value <= s_30_0
        fn_state.return_value = s_30_0;
        // N s_30_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var return_value <= s_31_0
        fn_state.return_value = s_31_0;
        // N s_31_2: jump b13
        return block_13(state, tracer, fn_state);
    }
}
