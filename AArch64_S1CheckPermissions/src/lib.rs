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
use u__IMPDEF_boolean::*;
use AArch64_S1ComputePermissions::*;
use ConstrainUnpredictable::*;
use HaveGCS::*;
use common::*;
pub fn AArch64_S1CheckPermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    walkstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeef284266e139aee2,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        gs_17367: bool,
        gs_17366: bool,
        gs_17373: bool,
        gs_17350: bool,
        gs_17348: bool,
        gs_17389: bool,
        gs_17345: bool,
        gs_17344: bool,
        gs_17376: bool,
        gs_17351: bool,
        permissions: ProductTypebf05c51f33174538,
        gs_17385: bool,
        gs_17352: bool,
        gs_17349: bool,
        gs_17392: bool,
        gs_17374: bool,
        ga_12907: ProductTypef170cab34335b70c,
        gs_17381: bool,
        gs_17384: bool,
        gs_17371: bool,
        gs_17386: bool,
        gs_17372: bool,
        gs_17391: bool,
        s1perms: ProductTypea231b9ca5c98dc3c,
        ga_12820: ProductTypef170cab34335b70c,
        gs_17383: bool,
        gs_17346: bool,
        gs_17355: bool,
        gs_17354: bool,
        gs_17382: bool,
        gs_17379: bool,
        gs_17375: bool,
        gs_17347: bool,
        gs_17380: bool,
        gs_17353: bool,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        walkstate: ProductType96e7acababe246a1,
        walkparams: ProductTypeef284266e139aee2,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
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
    ) -> ProductType1d757adad216cdef {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // D s_0_2: read-var walkstate.9:struct
        let s_0_2: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_0_3: write-var permissions <= s_0_2
        fn_state.permissions = s_0_2;
        // D s_0_4: read-var regime:u32
        let s_0_4: u32 = fn_state.regime;
        // D s_0_5: read-var walkstate:struct
        let s_0_5: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_0_6: read-var walkparams:struct
        let s_0_6: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_0_7: read-var accdesc:struct
        let s_0_7: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_8: call AArch64_S1ComputePermissions(s_0_4, s_0_5, s_0_6, s_0_7)
        let s_0_8: ProductTypea231b9ca5c98dc3c = AArch64_S1ComputePermissions(
            state,
            tracer,
            s_0_4,
            s_0_5,
            s_0_6,
            s_0_7,
        );
        // D s_0_9: write-var s1perms <= s_0_8
        fn_state.s1perms = s_0_8;
        // D s_0_10: read-var accdesc.1:struct
        let s_0_10: u32 = fn_state.accdesc._1;
        // C s_0_11: const #0u : u32
        let s_0_11: u32 = 0;
        // D s_0_12: cmp-eq s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) == (s_0_11));
        // N s_0_13: branch s_0_12 b139 b1
        if s_0_12 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_1_0: read-var accdesc.1:struct
        let s_1_0: u32 = fn_state.accdesc._1;
        // C s_1_1: const #6u : u32
        let s_1_1: u32 = 6;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b95 b2
        if s_1_2 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var accdesc.1:struct
        let s_2_0: u32 = fn_state.accdesc._1;
        // C s_2_1: const #5u : u32
        let s_2_1: u32 = 5;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b65 b3
        if s_2_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveGCS(s_3_0)
        let s_3_1: bool = HaveGCS(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b64 b4
        if s_3_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#17344 <= s_4_0
        fn_state.gs_17344 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_5_0: read-var gs#17344:u8
        let s_5_0: bool = fn_state.gs_17344;
        // N s_5_1: branch s_5_0 b53 b6
        if s_5_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_6_0: read-var accdesc.23:struct
        let s_6_0: bool = fn_state.accdesc._23;
        // N s_6_1: branch s_6_0 b52 b7
        if s_6_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#17345 <= s_7_0
        fn_state.gs_17345 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var gs#17345:u8
        let s_8_0: bool = fn_state.gs_17345;
        // N s_8_1: branch s_8_0 b51 b9
        if s_8_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#17346 <= s_9_0
        fn_state.gs_17346 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_10_0: read-var gs#17346:u8
        let s_10_0: bool = fn_state.gs_17346;
        // N s_10_1: branch s_10_0 b50 b11
        if s_10_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_11_0: read-var accdesc.32:struct
        let s_11_0: bool = fn_state.accdesc._32;
        // N s_11_1: branch s_11_0 b49 b12
        if s_11_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#17347 <= s_12_0
        fn_state.gs_17347 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_13_0: read-var gs#17347:u8
        let s_13_0: bool = fn_state.gs_17347;
        // N s_13_1: branch s_13_0 b48 b14
        if s_13_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#17348 <= s_14_0
        fn_state.gs_17348 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_15_0: read-var gs#17348:u8
        let s_15_0: bool = fn_state.gs_17348;
        // N s_15_1: branch s_15_0 b47 b16
        if s_15_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_16_0: read-var accdesc.23:struct
        let s_16_0: bool = fn_state.accdesc._23;
        // N s_16_1: branch s_16_0 b46 b17
        if s_16_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#17349 <= s_17_0
        fn_state.gs_17349 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_18_0: read-var gs#17349:u8
        let s_18_0: bool = fn_state.gs_17349;
        // N s_18_1: branch s_18_0 b45 b19
        if s_18_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_19_0: read-var accdesc.32:struct
        let s_19_0: bool = fn_state.accdesc._32;
        // N s_19_1: branch s_19_0 b44 b20
        if s_19_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#17350 <= s_20_0
        fn_state.gs_17350 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_21_0: read-var gs#17350:u8
        let s_21_0: bool = fn_state.gs_17350;
        // N s_21_1: branch s_21_0 b43 b22
        if s_21_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_22_0: read-var accdesc.32:struct
        let s_22_0: bool = fn_state.accdesc._32;
        // N s_22_1: branch s_22_0 b42 b23
        if s_22_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#17351 <= s_23_0
        fn_state.gs_17351 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_24_0: read-var gs#17351:u8
        let s_24_0: bool = fn_state.gs_17351;
        // N s_24_1: branch s_24_0 b41 b25
        if s_24_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#17352 <= s_25_0
        fn_state.gs_17352 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_26_0: read-var gs#17352:u8
        let s_26_0: bool = fn_state.gs_17352;
        // N s_26_1: branch s_26_0 b40 b27
        if s_26_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_27_0: read-var accdesc.32:struct
        let s_27_0: bool = fn_state.accdesc._32;
        // N s_27_1: branch s_27_0 b39 b28
        if s_27_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#17353 <= s_28_0
        fn_state.gs_17353 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_29_0: read-var gs#17353:u8
        let s_29_0: bool = fn_state.gs_17353;
        // N s_29_1: branch s_29_0 b38 b30
        if s_29_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#17354 <= s_30_0
        fn_state.gs_17354 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_31_0: read-var gs#17354:u8
        let s_31_0: bool = fn_state.gs_17354;
        // N s_31_1: branch s_31_0 b37 b32
        if s_31_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#17355 <= s_32_0
        fn_state.gs_17355 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_33_0: read-var gs#17355:u8
        let s_33_0: bool = fn_state.gs_17355;
        // N s_33_1: branch s_33_0 b36 b34
        if s_33_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_35_0: read-var fault:struct
        let s_35_0: ProductType1d757adad216cdef = fn_state.fault;
        // N s_35_1: return s_35_0
        return s_35_0;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_36_0: const #5u : u32
        let s_36_0: u32 = 5;
        // D s_36_1: write-var fault.16 <= s_36_0
        fn_state.fault._16 = s_36_0;
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // D s_36_3: write-var fault.3 <= s_36_2
        fn_state.fault._3 = s_36_2;
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // D s_36_5: write-var fault.19 <= s_36_4
        fn_state.fault._19 = s_36_4;
        // N s_36_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_37_0: read-var permissions.2:struct
        let s_37_0: bool = fn_state.permissions._2;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#17355 <= s_37_4
        fn_state.gs_17355 = s_37_4;
        // N s_37_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_38_0: read-var walkparams.24:struct
        let s_38_0: bool = fn_state.walkparams._24;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#17354 <= s_38_4
        fn_state.gs_17354 = s_38_4;
        // N s_38_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_39_0: read-var walkparams.12:struct
        let s_39_0: bool = fn_state.walkparams._12;
        // D s_39_1: read-var walkparams.14:struct
        let s_39_1: bool = fn_state.walkparams._14;
        // D s_39_2: cast zx s_39_0 -> bv
        let s_39_2: Bits = Bits::new(s_39_0 as u128, 1u16);
        // D s_39_3: cast zx s_39_1 -> bv
        let s_39_3: Bits = Bits::new(s_39_1 as u128, 1u16);
        // D s_39_4: cast reint s_39_2 -> u128
        let s_39_4: u128 = (s_39_2.value() as u128);
        // D s_39_5: size-of s_39_2
        let s_39_5: u16 = s_39_2.length();
        // D s_39_6: cast reint s_39_3 -> u128
        let s_39_6: u128 = (s_39_3.value() as u128);
        // D s_39_7: size-of s_39_3
        let s_39_7: u16 = s_39_3.length();
        // D s_39_8: lsl s_39_4 s_39_7
        let s_39_8: u128 = s_39_4 << s_39_7;
        // D s_39_9: or s_39_8 s_39_6
        let s_39_9: u128 = ((s_39_8) | (s_39_6));
        // D s_39_10: add s_39_5 s_39_7
        let s_39_10: u16 = (s_39_5 + s_39_7);
        // D s_39_11: create-bits s_39_9 s_39_10
        let s_39_11: Bits = Bits::new(s_39_9, s_39_10);
        // D s_39_12: cast reint s_39_11 -> u8
        let s_39_12: u8 = (s_39_11.value() as u8);
        // D s_39_13: cast zx s_39_12 -> bv
        let s_39_13: Bits = Bits::new(s_39_12 as u128, 2u16);
        // C s_39_14: const #3u : u8
        let s_39_14: u8 = 3;
        // C s_39_15: cast zx s_39_14 -> bv
        let s_39_15: Bits = Bits::new(s_39_14 as u128, 2u16);
        // D s_39_16: cmp-eq s_39_13 s_39_15
        let s_39_16: bool = ((s_39_13) == (s_39_15));
        // D s_39_17: not s_39_16
        let s_39_17: bool = !s_39_16;
        // D s_39_18: write-var gs#17353 <= s_39_17
        fn_state.gs_17353 = s_39_17;
        // N s_39_19: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_40_0: const #5u : u32
        let s_40_0: u32 = 5;
        // D s_40_1: write-var fault.16 <= s_40_0
        fn_state.fault._16 = s_40_0;
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // D s_40_3: write-var fault.19 <= s_40_2
        fn_state.fault._19 = s_40_2;
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // D s_40_5: write-var fault.13 <= s_40_4
        fn_state.fault._13 = s_40_4;
        // N s_40_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_41_0: read-var walkstate.7:struct
        let s_41_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_41_1: write-var ga#12907 <= s_41_0
        fn_state.ga_12907 = s_41_0;
        // D s_41_2: read-var ga#12907.6:struct
        let s_41_2: u32 = fn_state.ga_12907._6;
        // C s_41_3: const #2u : u32
        let s_41_3: u32 = 2;
        // D s_41_4: cmp-eq s_41_2 s_41_3
        let s_41_4: bool = ((s_41_2) == (s_41_3));
        // D s_41_5: write-var gs#17352 <= s_41_4
        fn_state.gs_17352 = s_41_4;
        // N s_41_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_42_0: read-var accdesc.27:struct
        let s_42_0: bool = fn_state.accdesc._27;
        // D s_42_1: write-var gs#17351 <= s_42_0
        fn_state.gs_17351 = s_42_0;
        // N s_42_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_43_0: const #5u : u32
        let s_43_0: u32 = 5;
        // D s_43_1: write-var fault.16 <= s_43_0
        fn_state.fault._16 = s_43_0;
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // D s_43_3: write-var fault.19 <= s_43_2
        fn_state.fault._19 = s_43_2;
        // N s_43_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_44_0: read-var s1perms.6:struct
        let s_44_0: bool = fn_state.s1perms._6;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#17350 <= s_44_4
        fn_state.gs_17350 = s_44_4;
        // N s_44_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_45_0: const #5u : u32
        let s_45_0: u32 = 5;
        // D s_45_1: write-var fault.16 <= s_45_0
        fn_state.fault._16 = s_45_0;
        // C s_45_2: const #0u : u8
        let s_45_2: bool = false;
        // D s_45_3: write-var fault.19 <= s_45_2
        fn_state.fault._19 = s_45_2;
        // N s_45_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_46_0: read-var s1perms.5:struct
        let s_46_0: bool = fn_state.s1perms._5;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #0u : u8
        let s_46_2: bool = false;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#17349 <= s_46_4
        fn_state.gs_17349 = s_46_4;
        // N s_46_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_47_0: const #5u : u32
        let s_47_0: u32 = 5;
        // D s_47_1: write-var fault.16 <= s_47_0
        fn_state.fault._16 = s_47_0;
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // D s_47_3: write-var fault.11 <= s_47_2
        fn_state.fault._11 = s_47_2;
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // D s_47_5: write-var fault.19 <= s_47_4
        fn_state.fault._19 = s_47_4;
        // N s_47_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_48_0: read-var s1perms.3:struct
        let s_48_0: bool = fn_state.s1perms._3;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#17348 <= s_48_4
        fn_state.gs_17348 = s_48_4;
        // N s_48_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_49_0: read-var s1perms.2:struct
        let s_49_0: bool = fn_state.s1perms._2;
        // D s_49_1: write-var gs#17347 <= s_49_0
        fn_state.gs_17347 = s_49_0;
        // N s_49_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_50_0: const #5u : u32
        let s_50_0: u32 = 5;
        // D s_50_1: write-var fault.16 <= s_50_0
        fn_state.fault._16 = s_50_0;
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // D s_50_3: write-var fault.11 <= s_50_2
        fn_state.fault._11 = s_50_2;
        // C s_50_4: const #0u : u8
        let s_50_4: bool = false;
        // D s_50_5: write-var fault.19 <= s_50_4
        fn_state.fault._19 = s_50_4;
        // N s_50_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_51_0: read-var s1perms.1:struct
        let s_51_0: bool = fn_state.s1perms._1;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#17346 <= s_51_4
        fn_state.gs_17346 = s_51_4;
        // N s_51_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_52_0: read-var s1perms.2:struct
        let s_52_0: bool = fn_state.s1perms._2;
        // D s_52_1: write-var gs#17345 <= s_52_0
        fn_state.gs_17345 = s_52_0;
        // N s_52_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_53_0: read-var s1perms.0:struct
        let s_53_0: bool = fn_state.s1perms._0;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #0u : u8
        let s_53_2: bool = false;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // N s_53_5: branch s_53_4 b63 b54
        if s_53_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_54_0: read-var accdesc.32:struct
        let s_54_0: bool = fn_state.accdesc._32;
        // N s_54_1: branch s_54_0 b62 b55
        if s_54_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#17366 <= s_55_0
        fn_state.gs_17366 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_56_0: read-var gs#17366:u8
        let s_56_0: bool = fn_state.gs_17366;
        // N s_56_1: branch s_56_0 b61 b57
        if s_56_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#17367 <= s_57_0
        fn_state.gs_17367 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_58_0: read-var gs#17367:u8
        let s_58_0: bool = fn_state.gs_17367;
        // N s_58_1: branch s_58_0 b60 b59
        if s_58_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_59_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_60_0: const #5u : u32
        let s_60_0: u32 = 5;
        // D s_60_1: write-var fault.16 <= s_60_0
        fn_state.fault._16 = s_60_0;
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // D s_60_3: write-var fault.3 <= s_60_2
        fn_state.fault._3 = s_60_2;
        // C s_60_4: const #1u : u8
        let s_60_4: bool = true;
        // D s_60_5: write-var fault.19 <= s_60_4
        fn_state.fault._19 = s_60_4;
        // N s_60_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_61_0: read-var permissions.2:struct
        let s_61_0: bool = fn_state.permissions._2;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#17367 <= s_61_4
        fn_state.gs_17367 = s_61_4;
        // N s_61_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_62_0: read-var walkparams.12:struct
        let s_62_0: bool = fn_state.walkparams._12;
        // D s_62_1: read-var walkparams.14:struct
        let s_62_1: bool = fn_state.walkparams._14;
        // D s_62_2: cast zx s_62_0 -> bv
        let s_62_2: Bits = Bits::new(s_62_0 as u128, 1u16);
        // D s_62_3: cast zx s_62_1 -> bv
        let s_62_3: Bits = Bits::new(s_62_1 as u128, 1u16);
        // D s_62_4: cast reint s_62_2 -> u128
        let s_62_4: u128 = (s_62_2.value() as u128);
        // D s_62_5: size-of s_62_2
        let s_62_5: u16 = s_62_2.length();
        // D s_62_6: cast reint s_62_3 -> u128
        let s_62_6: u128 = (s_62_3.value() as u128);
        // D s_62_7: size-of s_62_3
        let s_62_7: u16 = s_62_3.length();
        // D s_62_8: lsl s_62_4 s_62_7
        let s_62_8: u128 = s_62_4 << s_62_7;
        // D s_62_9: or s_62_8 s_62_6
        let s_62_9: u128 = ((s_62_8) | (s_62_6));
        // D s_62_10: add s_62_5 s_62_7
        let s_62_10: u16 = (s_62_5 + s_62_7);
        // D s_62_11: create-bits s_62_9 s_62_10
        let s_62_11: Bits = Bits::new(s_62_9, s_62_10);
        // D s_62_12: cast reint s_62_11 -> u8
        let s_62_12: u8 = (s_62_11.value() as u8);
        // D s_62_13: cast zx s_62_12 -> bv
        let s_62_13: Bits = Bits::new(s_62_12 as u128, 2u16);
        // C s_62_14: const #3u : u8
        let s_62_14: u8 = 3;
        // C s_62_15: cast zx s_62_14 -> bv
        let s_62_15: Bits = Bits::new(s_62_14 as u128, 2u16);
        // D s_62_16: cmp-ne s_62_13 s_62_15
        let s_62_16: bool = ((s_62_13) != (s_62_15));
        // D s_62_17: write-var gs#17366 <= s_62_16
        fn_state.gs_17366 = s_62_16;
        // N s_62_18: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_63_0: const #5u : u32
        let s_63_0: u32 = 5;
        // D s_63_1: write-var fault.16 <= s_63_0
        fn_state.fault._16 = s_63_0;
        // N s_63_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_64_0: read-var accdesc.1:struct
        let s_64_0: u32 = fn_state.accdesc._1;
        // C s_64_1: const #11u : u32
        let s_64_1: u32 = 11;
        // D s_64_2: cmp-eq s_64_0 s_64_1
        let s_64_2: bool = ((s_64_0) == (s_64_1));
        // D s_64_3: write-var gs#17344 <= s_64_2
        fn_state.gs_17344 = s_64_2;
        // N s_64_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_65_0: read-var accdesc.8:struct
        let s_65_0: u8 = fn_state.accdesc._8;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 2u16);
        // C s_65_2: const #448u : u32
        let s_65_2: u32 = 448;
        // D s_65_3: read-reg s_65_2:u8
        let s_65_3: u8 = {
            let value = state.read_register::<u8>(s_65_2 as isize);
            tracer.read_register(s_65_2 as isize, value);
            value
        };
        // D s_65_4: cast zx s_65_3 -> bv
        let s_65_4: Bits = Bits::new(s_65_3 as u128, 2u16);
        // D s_65_5: cmp-eq s_65_1 s_65_4
        let s_65_5: bool = ((s_65_1) == (s_65_4));
        // N s_65_6: branch s_65_5 b68 b66
        if s_65_5 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_66_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_67_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_68_0: read-var s1perms.2:struct
        let s_68_0: bool = fn_state.s1perms._2;
        // N s_68_1: branch s_68_0 b94 b69
        if s_68_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#17371 <= s_69_0
        fn_state.gs_17371 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_70_0: read-var gs#17371:u8
        let s_70_0: bool = fn_state.gs_17371;
        // N s_70_1: branch s_70_0 b93 b71
        if s_70_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#17372 <= s_71_0
        fn_state.gs_17372 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_72_0: read-var gs#17372:u8
        let s_72_0: bool = fn_state.gs_17372;
        // N s_72_1: branch s_72_0 b92 b73
        if s_72_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_73_0: read-var walkparams.2:struct
        let s_73_0: bool = fn_state.walkparams._2;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // N s_73_5: branch s_73_4 b91 b74
        if s_73_4 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#17373 <= s_74_0
        fn_state.gs_17373 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_75_0: read-var gs#17373:u8
        let s_75_0: bool = fn_state.gs_17373;
        // N s_75_1: branch s_75_0 b90 b76
        if s_75_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#17374 <= s_76_0
        fn_state.gs_17374 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_77_0: read-var gs#17374:u8
        let s_77_0: bool = fn_state.gs_17374;
        // N s_77_1: branch s_77_0 b89 b78
        if s_77_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_78_0: read-var s1perms.5:struct
        let s_78_0: bool = fn_state.s1perms._5;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // N s_78_5: branch s_78_4 b88 b79
        if s_78_4 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#17375 <= s_79_0
        fn_state.gs_17375 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_80_0: read-var gs#17375:u8
        let s_80_0: bool = fn_state.gs_17375;
        // N s_80_1: branch s_80_0 b87 b81
        if s_80_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_81_0: read-var walkparams.2:struct
        let s_81_0: bool = fn_state.walkparams._2;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // N s_81_5: branch s_81_4 b86 b82
        if s_81_4 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#17376 <= s_82_0
        fn_state.gs_17376 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_83_0: read-var gs#17376:u8
        let s_83_0: bool = fn_state.gs_17376;
        // N s_83_1: branch s_83_0 b85 b84
        if s_83_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_84_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_85_0: const #5u : u32
        let s_85_0: u32 = 5;
        // D s_85_1: write-var fault.16 <= s_85_0
        fn_state.fault._16 = s_85_0;
        // N s_85_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_86_0: read-var s1perms.6:struct
        let s_86_0: bool = fn_state.s1perms._6;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #0u : u8
        let s_86_2: bool = false;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#17376 <= s_86_4
        fn_state.gs_17376 = s_86_4;
        // N s_86_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_87_0: const #5u : u32
        let s_87_0: u32 = 5;
        // D s_87_1: write-var fault.16 <= s_87_0
        fn_state.fault._16 = s_87_0;
        // N s_87_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_88_0: const #"Permission fault on EL0 IC_IVAU execution" : str
        let s_88_0: &'static str = "Permission fault on EL0 IC_IVAU execution";
        // S s_88_1: call __IMPDEF_boolean(s_88_0)
        let s_88_1: bool = u__IMPDEF_boolean(state, tracer, s_88_0);
        // D s_88_2: write-var gs#17375 <= s_88_1
        fn_state.gs_17375 = s_88_1;
        // N s_88_3: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_89_0: const #5u : u32
        let s_89_0: u32 = 5;
        // D s_89_1: write-var fault.16 <= s_89_0
        fn_state.fault._16 = s_89_0;
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // D s_89_3: write-var fault.11 <= s_89_2
        fn_state.fault._11 = s_89_2;
        // N s_89_4: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_90_0: read-var s1perms.3:struct
        let s_90_0: bool = fn_state.s1perms._3;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #0u : u8
        let s_90_2: bool = false;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#17374 <= s_90_4
        fn_state.gs_17374 = s_90_4;
        // N s_90_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_91_0: read-var s1perms.2:struct
        let s_91_0: bool = fn_state.s1perms._2;
        // D s_91_1: write-var gs#17373 <= s_91_0
        fn_state.gs_17373 = s_91_0;
        // N s_91_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_92_0: const #5u : u32
        let s_92_0: u32 = 5;
        // D s_92_1: write-var fault.16 <= s_92_0
        fn_state.fault._16 = s_92_0;
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // D s_92_3: write-var fault.11 <= s_92_2
        fn_state.fault._11 = s_92_2;
        // N s_92_4: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_93_0: const #"Permission fault on EL0 IC_IVAU execution" : str
        let s_93_0: &'static str = "Permission fault on EL0 IC_IVAU execution";
        // S s_93_1: call __IMPDEF_boolean(s_93_0)
        let s_93_1: bool = u__IMPDEF_boolean(state, tracer, s_93_0);
        // D s_93_2: write-var gs#17372 <= s_93_1
        fn_state.gs_17372 = s_93_1;
        // N s_93_3: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_94_0: read-var s1perms.1:struct
        let s_94_0: bool = fn_state.s1perms._1;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #0u : u8
        let s_94_2: bool = false;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // D s_94_5: write-var gs#17371 <= s_94_4
        fn_state.gs_17371 = s_94_4;
        // N s_94_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_95_0: read-var accdesc.5:struct
        let s_95_0: u32 = fn_state.accdesc._5;
        // C s_95_1: const #1u : u32
        let s_95_1: u32 = 1;
        // D s_95_2: cmp-eq s_95_0 s_95_1
        let s_95_2: bool = ((s_95_0) == (s_95_1));
        // N s_95_3: branch s_95_2 b131 b96
        if s_95_2 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_96_0: read-var accdesc.8:struct
        let s_96_0: u8 = fn_state.accdesc._8;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 2u16);
        // C s_96_2: const #448u : u32
        let s_96_2: u32 = 448;
        // D s_96_3: read-reg s_96_2:u8
        let s_96_3: u8 = {
            let value = state.read_register::<u8>(s_96_2 as isize);
            tracer.read_register(s_96_2 as isize, value);
            value
        };
        // D s_96_4: cast zx s_96_3 -> bv
        let s_96_4: Bits = Bits::new(s_96_3 as u128, 2u16);
        // D s_96_5: cmp-eq s_96_1 s_96_4
        let s_96_5: bool = ((s_96_1) == (s_96_4));
        // N s_96_6: branch s_96_5 b98 b97
        if s_96_5 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_97_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_98_0: read-var s1perms.2:struct
        let s_98_0: bool = fn_state.s1perms._2;
        // N s_98_1: branch s_98_0 b130 b99
        if s_98_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#17379 <= s_99_0
        fn_state.gs_17379 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_100_0: read-var gs#17379:u8
        let s_100_0: bool = fn_state.gs_17379;
        // N s_100_1: branch s_100_0 b129 b101
        if s_100_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_101_0: read-var walkparams.2:struct
        let s_101_0: bool = fn_state.walkparams._2;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // N s_101_5: branch s_101_4 b128 b102
        if s_101_4 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#17380 <= s_102_0
        fn_state.gs_17380 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_103_0: read-var gs#17380:u8
        let s_103_0: bool = fn_state.gs_17380;
        // N s_103_1: branch s_103_0 b127 b104
        if s_103_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#17381 <= s_104_0
        fn_state.gs_17381 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_105_0: read-var gs#17381:u8
        let s_105_0: bool = fn_state.gs_17381;
        // N s_105_1: branch s_105_0 b126 b106
        if s_105_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#17382 <= s_106_0
        fn_state.gs_17382 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_107_0: read-var gs#17382:u8
        let s_107_0: bool = fn_state.gs_17382;
        // N s_107_1: branch s_107_0 b125 b108
        if s_107_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#17383 <= s_108_0
        fn_state.gs_17383 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_109_0: read-var gs#17383:u8
        let s_109_0: bool = fn_state.gs_17383;
        // N s_109_1: branch s_109_0 b124 b110
        if s_109_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_110_0: read-var s1perms.5:struct
        let s_110_0: bool = fn_state.s1perms._5;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #0u : u8
        let s_110_2: bool = false;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // N s_110_5: branch s_110_4 b123 b111
        if s_110_4 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_111_0: read-var walkparams.2:struct
        let s_111_0: bool = fn_state.walkparams._2;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // N s_111_5: branch s_111_4 b122 b112
        if s_111_4 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#17384 <= s_112_0
        fn_state.gs_17384 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_113_0: read-var gs#17384:u8
        let s_113_0: bool = fn_state.gs_17384;
        // N s_113_1: branch s_113_0 b121 b114
        if s_113_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#17385 <= s_114_0
        fn_state.gs_17385 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_115_0: read-var gs#17385:u8
        let s_115_0: bool = fn_state.gs_17385;
        // N s_115_1: branch s_115_0 b120 b116
        if s_115_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#17386 <= s_116_0
        fn_state.gs_17386 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_117_0: read-var gs#17386:u8
        let s_117_0: bool = fn_state.gs_17386;
        // N s_117_1: branch s_117_0 b119 b118
        if s_117_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_118_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_119_0: const #5u : u32
        let s_119_0: u32 = 5;
        // D s_119_1: write-var fault.16 <= s_119_0
        fn_state.fault._16 = s_119_0;
        // N s_119_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_120_0: read-var s1perms.6:struct
        let s_120_0: bool = fn_state.s1perms._6;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #0u : u8
        let s_120_2: bool = false;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#17386 <= s_120_4
        fn_state.gs_17386 = s_120_4;
        // N s_120_6: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_121_0: read-var accdesc.5:struct
        let s_121_0: u32 = fn_state.accdesc._5;
        // C s_121_1: const #2u : u32
        let s_121_1: u32 = 2;
        // D s_121_2: cmp-eq s_121_0 s_121_1
        let s_121_2: bool = ((s_121_0) == (s_121_1));
        // D s_121_3: write-var gs#17385 <= s_121_2
        fn_state.gs_17385 = s_121_2;
        // N s_121_4: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_122_0: read-var accdesc.19:struct
        let s_122_0: u32 = fn_state.accdesc._19;
        // C s_122_1: const #2u : u32
        let s_122_1: u32 = 2;
        // D s_122_2: cmp-eq s_122_0 s_122_1
        let s_122_2: bool = ((s_122_0) == (s_122_1));
        // D s_122_3: write-var gs#17384 <= s_122_2
        fn_state.gs_17384 = s_122_2;
        // N s_122_4: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_123_0: const #5u : u32
        let s_123_0: u32 = 5;
        // D s_123_1: write-var fault.16 <= s_123_0
        fn_state.fault._16 = s_123_0;
        // N s_123_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_124_0: const #5u : u32
        let s_124_0: u32 = 5;
        // D s_124_1: write-var fault.16 <= s_124_0
        fn_state.fault._16 = s_124_0;
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // D s_124_3: write-var fault.11 <= s_124_2
        fn_state.fault._11 = s_124_2;
        // N s_124_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_125_0: read-var s1perms.3:struct
        let s_125_0: bool = fn_state.s1perms._3;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #0u : u8
        let s_125_2: bool = false;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#17383 <= s_125_4
        fn_state.gs_17383 = s_125_4;
        // N s_125_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_126_0: read-var s1perms.2:struct
        let s_126_0: bool = fn_state.s1perms._2;
        // D s_126_1: write-var gs#17382 <= s_126_0
        fn_state.gs_17382 = s_126_0;
        // N s_126_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_127_0: read-var accdesc.5:struct
        let s_127_0: u32 = fn_state.accdesc._5;
        // C s_127_1: const #2u : u32
        let s_127_1: u32 = 2;
        // D s_127_2: cmp-eq s_127_0 s_127_1
        let s_127_2: bool = ((s_127_0) == (s_127_1));
        // D s_127_3: write-var gs#17381 <= s_127_2
        fn_state.gs_17381 = s_127_2;
        // N s_127_4: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_128_0: read-var accdesc.19:struct
        let s_128_0: u32 = fn_state.accdesc._19;
        // C s_128_1: const #2u : u32
        let s_128_1: u32 = 2;
        // D s_128_2: cmp-eq s_128_0 s_128_1
        let s_128_2: bool = ((s_128_0) == (s_128_1));
        // D s_128_3: write-var gs#17380 <= s_128_2
        fn_state.gs_17380 = s_128_2;
        // N s_128_4: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_129_0: const #5u : u32
        let s_129_0: u32 = 5;
        // D s_129_1: write-var fault.16 <= s_129_0
        fn_state.fault._16 = s_129_0;
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // D s_129_3: write-var fault.11 <= s_129_2
        fn_state.fault._11 = s_129_2;
        // N s_129_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_130_0: read-var s1perms.1:struct
        let s_130_0: bool = fn_state.s1perms._1;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #0u : u8
        let s_130_2: bool = false;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#17379 <= s_130_4
        fn_state.gs_17379 = s_130_4;
        // N s_130_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_131_0: read-var s1perms.2:struct
        let s_131_0: bool = fn_state.s1perms._2;
        // N s_131_1: branch s_131_0 b138 b132
        if s_131_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#17389 <= s_132_0
        fn_state.gs_17389 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_133_0: read-var gs#17389:u8
        let s_133_0: bool = fn_state.gs_17389;
        // N s_133_1: branch s_133_0 b137 b134
        if s_133_0 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_134_0: read-var s1perms.6:struct
        let s_134_0: bool = fn_state.s1perms._6;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #0u : u8
        let s_134_2: bool = false;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // N s_134_5: branch s_134_4 b136 b135
        if s_134_4 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_135_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_136_0: const #5u : u32
        let s_136_0: u32 = 5;
        // D s_136_1: write-var fault.16 <= s_136_0
        fn_state.fault._16 = s_136_0;
        // N s_136_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_137_0: const #5u : u32
        let s_137_0: u32 = 5;
        // D s_137_1: write-var fault.16 <= s_137_0
        fn_state.fault._16 = s_137_0;
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // D s_137_3: write-var fault.11 <= s_137_2
        fn_state.fault._11 = s_137_2;
        // N s_137_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_138_0: read-var s1perms.3:struct
        let s_138_0: bool = fn_state.s1perms._3;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #0u : u8
        let s_138_2: bool = false;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#17389 <= s_138_4
        fn_state.gs_17389 = s_138_4;
        // N s_138_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_139_0: read-var s1perms.2:struct
        let s_139_0: bool = fn_state.s1perms._2;
        // N s_139_1: branch s_139_0 b151 b140
        if s_139_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#17391 <= s_140_0
        fn_state.gs_17391 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_141_0: read-var gs#17391:u8
        let s_141_0: bool = fn_state.gs_17391;
        // N s_141_1: branch s_141_0 b150 b142
        if s_141_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_142_0: read-var walkstate.7:struct
        let s_142_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_142_1: write-var ga#12820 <= s_142_0
        fn_state.ga_12820 = s_142_0;
        // D s_142_2: read-var ga#12820.2:struct
        let s_142_2: u32 = fn_state.ga_12820._2;
        // C s_142_3: const #1u : u32
        let s_142_3: u32 = 1;
        // D s_142_4: cmp-eq s_142_2 s_142_3
        let s_142_4: bool = ((s_142_2) == (s_142_3));
        // N s_142_5: branch s_142_4 b149 b143
        if s_142_4 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#17392 <= s_143_0
        fn_state.gs_17392 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_144_0: read-var gs#17392:u8
        let s_144_0: bool = fn_state.gs_17392;
        // N s_144_1: branch s_144_0 b148 b145
        if s_144_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_145_0: read-var s1perms.8:struct
        let s_145_0: bool = fn_state.s1perms._8;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #0u : u8
        let s_145_2: bool = false;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // N s_145_5: branch s_145_4 b147 b146
        if s_145_4 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_146_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_147_0: const #5u : u32
        let s_147_0: u32 = 5;
        // D s_147_1: write-var fault.16 <= s_147_0
        fn_state.fault._16 = s_147_0;
        // N s_147_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_148_0: const #5u : u32
        let s_148_0: u32 = 5;
        // D s_148_1: write-var fault.16 <= s_148_0
        fn_state.fault._16 = s_148_0;
        // N s_148_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_149_0: const #7u : u32
        let s_149_0: u32 = 7;
        // S s_149_1: call ConstrainUnpredictable(s_149_0)
        let s_149_1: u32 = ConstrainUnpredictable(state, tracer, s_149_0);
        // C s_149_2: const #12u : u32
        let s_149_2: u32 = 12;
        // S s_149_3: cmp-eq s_149_1 s_149_2
        let s_149_3: bool = ((s_149_1) == (s_149_2));
        // D s_149_4: write-var gs#17392 <= s_149_3
        fn_state.gs_17392 = s_149_3;
        // N s_149_5: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_150_0: const #5u : u32
        let s_150_0: u32 = 5;
        // D s_150_1: write-var fault.16 <= s_150_0
        fn_state.fault._16 = s_150_0;
        // C s_150_2: const #1u : u8
        let s_150_2: bool = true;
        // D s_150_3: write-var fault.11 <= s_150_2
        fn_state.fault._11 = s_150_2;
        // N s_150_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_151_0: read-var s1perms.4:struct
        let s_151_0: bool = fn_state.s1perms._4;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #0u : u8
        let s_151_2: bool = false;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#17391 <= s_151_4
        fn_state.gs_17391 = s_151_4;
        // N s_151_6: jump b141
        return block_141(state, tracer, fn_state);
    }
}
