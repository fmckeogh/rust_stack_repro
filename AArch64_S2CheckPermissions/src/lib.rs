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
use HaveGCS::*;
use u__IMPDEF_boolean::*;
use ConstrainUnpredictable::*;
use ELUsingAArch32::*;
use AArch64_S2ComputePermissions::*;
use u_get_VTCR_EL2_Type_GCSH::*;
use common::*;
pub fn AArch64_S2CheckPermissions<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    walkstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeb05ce25a107f0c5e,
    ipa: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType3b8bd97143a1dd5c {
    #[derive(Default)]
    struct FunctionState {
        gs_18791: bool,
        r: bool,
        gs_18819: bool,
        gs_18783: bool,
        gs_18827: bool,
        fault: ProductType1d757adad216cdef,
        gs_18815: bool,
        gs_18821: bool,
        gs_18846: bool,
        gs_18831: bool,
        gs_18866: bool,
        gs_18834: bool,
        gs_18805: bool,
        gs_18839: bool,
        ga_14077: ProductTypef170cab34335b70c,
        gs_18787: bool,
        gs_18829: bool,
        gs_18830: bool,
        gs_18850: bool,
        ga_13962: ProductTypeda0231e9dc169f81,
        gs_18786: bool,
        gs_18820: bool,
        gs_18845: bool,
        gs_18865: bool,
        gs_18798: bool,
        permissions: ProductTypebf05c51f33174538,
        gs_18784: bool,
        gs_18824: bool,
        gs_18812: bool,
        w: bool,
        gs_18816: bool,
        or: bool,
        gs_18817: bool,
        gs_18864: bool,
        gs_18860: bool,
        gs_18849: bool,
        gs_18852: bool,
        ow: bool,
        gs_18794: bool,
        memtype: u32,
        gs_18857: bool,
        gs_18789: bool,
        gs_18861: bool,
        gs_18795: bool,
        gs_18867: bool,
        gs_18858: bool,
        ga_14111: ProductTypef170cab34335b70c,
        gs_18818: bool,
        ga_14001: ProductTypeda0231e9dc169f81,
        gs_18859: bool,
        gs_18813: bool,
        gs_18853: bool,
        s2perms: ProductType2fc9d3588999ac79,
        gs_18844: bool,
        gs_18862: bool,
        gs_18797: bool,
        gs_18796: bool,
        gs_18785: bool,
        gs_18828: bool,
        gs_18856: bool,
        gs_18788: bool,
        gs_18832: bool,
        gs_18792: bool,
        gs_18851: bool,
        gs_18793: bool,
        gs_18800: bool,
        gs_18825: bool,
        gs_18855: bool,
        mro: bool,
        gs_18840: bool,
        gs_18814: bool,
        gs_18835: bool,
        gs_18790: bool,
        gs_18863: bool,
        gs_18826: bool,
        gs_18799: bool,
        gs_18833: bool,
        gs_18854: bool,
        gs_18838: bool,
        fault_in: ProductType1d757adad216cdef,
        walkstate: ProductType96e7acababe246a1,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ipa: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        walkstate,
        walkparams,
        ipa,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_0_0: read-var walkstate.7:struct
        let s_0_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_0_1: write-var ga#14111 <= s_0_0
        fn_state.ga_14111 = s_0_0;
        // D s_0_2: read-var ga#14111.2:struct
        let s_0_2: u32 = fn_state.ga_14111._2;
        // D s_0_3: write-var memtype <= s_0_2
        fn_state.memtype = s_0_2;
        // D s_0_4: read-var walkstate.9:struct
        let s_0_4: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_0_5: write-var permissions <= s_0_4
        fn_state.permissions = s_0_4;
        // D s_0_6: read-var fault_in:struct
        let s_0_6: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_7: write-var fault <= s_0_6
        fn_state.fault = s_0_6;
        // D s_0_8: read-var permissions:struct
        let s_0_8: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_0_9: read-var walkparams:struct
        let s_0_9: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_0_10: read-var accdesc:struct
        let s_0_10: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_11: call AArch64_S2ComputePermissions(s_0_8, s_0_9, s_0_10)
        let s_0_11: ProductType2fc9d3588999ac79 = AArch64_S2ComputePermissions(
            state,
            tracer,
            s_0_8,
            s_0_9,
            s_0_10,
        );
        // D s_0_12: write-var s2perms <= s_0_11
        fn_state.s2perms = s_0_11;
        // D s_0_13: read-var accdesc.1:struct
        let s_0_13: u32 = fn_state.accdesc._1;
        // C s_0_14: const #13u : u32
        let s_0_14: u32 = 13;
        // D s_0_15: cmp-eq s_0_13 s_0_14
        let s_0_15: bool = ((s_0_13) == (s_0_14));
        // N s_0_16: branch s_0_15 b279 b1
        if s_0_15 {
            return block_279(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_1_0: read-var accdesc.21:struct
        let s_1_0: bool = fn_state.accdesc._21;
        // N s_1_1: branch s_1_0 b278 b2
        if s_1_0 {
            return block_278(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_2_0: read-var s2perms.8:struct
        let s_2_0: bool = fn_state.s2perms._8;
        // D s_2_1: write-var r <= s_2_0
        fn_state.r = s_2_0;
        // D s_2_2: read-var s2perms.13:struct
        let s_2_2: bool = fn_state.s2perms._13;
        // D s_2_3: write-var w <= s_2_2
        fn_state.w = s_2_2;
        // D s_2_4: read-var s2perms.0:struct
        let s_2_4: bool = fn_state.s2perms._0;
        // D s_2_5: write-var or <= s_2_4
        fn_state.or = s_2_4;
        // D s_2_6: read-var s2perms.4:struct
        let s_2_6: bool = fn_state.s2perms._4;
        // D s_2_7: write-var ow <= s_2_6
        fn_state.ow = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_3_0: read-var accdesc.1:struct
        let s_3_0: u32 = fn_state.accdesc._1;
        // C s_3_1: const #13u : u32
        let s_3_1: u32 = 13;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b202 b4
        if s_3_2 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_4_0: read-var walkstate.11:struct
        let s_4_0: bool = fn_state.walkstate._11;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b201 b5
        if s_4_4 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#18783 <= s_5_0
        fn_state.gs_18783 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_6_0: read-var gs#18783:u8
        let s_6_0: bool = fn_state.gs_18783;
        // N s_6_1: branch s_6_0 b200 b7
        if s_6_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_7_0: read-var walkstate.11:struct
        let s_7_0: bool = fn_state.walkstate._11;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-ne s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) != (s_7_3));
        // N s_7_5: branch s_7_4 b199 b8
        if s_7_4 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#18784 <= s_8_0
        fn_state.gs_18784 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_9_0: read-var gs#18784:u8
        let s_9_0: bool = fn_state.gs_18784;
        // N s_9_1: branch s_9_0 b198 b10
        if s_9_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#18785 <= s_10_0
        fn_state.gs_18785 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_11_0: read-var gs#18785:u8
        let s_11_0: bool = fn_state.gs_18785;
        // N s_11_1: branch s_11_0 b197 b12
        if s_11_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#18786 <= s_12_0
        fn_state.gs_18786 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_13_0: read-var gs#18786:u8
        let s_13_0: bool = fn_state.gs_18786;
        // N s_13_1: branch s_13_0 b196 b14
        if s_13_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#18787 <= s_14_0
        fn_state.gs_18787 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_15_0: read-var gs#18787:u8
        let s_15_0: bool = fn_state.gs_18787;
        // D s_15_1: write-var gs#18788 <= s_15_0
        fn_state.gs_18788 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_16_0: read-var gs#18788:u8
        let s_16_0: bool = fn_state.gs_18788;
        // N s_16_1: branch s_16_0 b195 b17
        if s_16_0 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_17_0: read-var accdesc.1:struct
        let s_17_0: u32 = fn_state.accdesc._1;
        // C s_17_1: const #0u : u32
        let s_17_1: u32 = 0;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b177 b18
        if s_17_2 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_18_0: read-var accdesc.1:struct
        let s_18_0: u32 = fn_state.accdesc._1;
        // C s_18_1: const #6u : u32
        let s_18_1: u32 = 6;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // N s_18_3: branch s_18_2 b115 b19
        if s_18_2 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_19_0: read-var accdesc.1:struct
        let s_19_0: u32 = fn_state.accdesc._1;
        // C s_19_1: const #5u : u32
        let s_19_1: u32 = 5;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b76 b20
        if s_19_2 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_20_0: read-var accdesc.23:struct
        let s_20_0: bool = fn_state.accdesc._23;
        // N s_20_1: branch s_20_0 b75 b21
        if s_20_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#18789 <= s_21_0
        fn_state.gs_18789 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_22_0: read-var gs#18789:u8
        let s_22_0: bool = fn_state.gs_18789;
        // N s_22_1: branch s_22_0 b74 b23
        if s_22_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#18790 <= s_23_0
        fn_state.gs_18790 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_24_0: read-var gs#18790:u8
        let s_24_0: bool = fn_state.gs_18790;
        // N s_24_1: branch s_24_0 b73 b25
        if s_24_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_25_0: read-var accdesc.32:struct
        let s_25_0: bool = fn_state.accdesc._32;
        // N s_25_1: branch s_25_0 b72 b26
        if s_25_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#18791 <= s_26_0
        fn_state.gs_18791 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_27_0: read-var gs#18791:u8
        let s_27_0: bool = fn_state.gs_18791;
        // N s_27_1: branch s_27_0 b71 b28
        if s_27_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#18792 <= s_28_0
        fn_state.gs_18792 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_29_0: read-var gs#18792:u8
        let s_29_0: bool = fn_state.gs_18792;
        // N s_29_1: branch s_29_0 b70 b30
        if s_29_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_30_0: read-var accdesc.23:struct
        let s_30_0: bool = fn_state.accdesc._23;
        // N s_30_1: branch s_30_0 b69 b31
        if s_30_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#18793 <= s_31_0
        fn_state.gs_18793 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_32_0: read-var gs#18793:u8
        let s_32_0: bool = fn_state.gs_18793;
        // N s_32_1: branch s_32_0 b68 b33
        if s_32_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_33_0: read-var accdesc.32:struct
        let s_33_0: bool = fn_state.accdesc._32;
        // N s_33_1: branch s_33_0 b67 b34
        if s_33_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#18794 <= s_34_0
        fn_state.gs_18794 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_35_0: read-var gs#18794:u8
        let s_35_0: bool = fn_state.gs_18794;
        // N s_35_1: branch s_35_0 b66 b36
        if s_35_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_36_0: read-var accdesc.27:struct
        let s_36_0: bool = fn_state.accdesc._27;
        // N s_36_1: branch s_36_0 b65 b37
        if s_36_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_37_0: read-var accdesc.28:struct
        let s_37_0: bool = fn_state.accdesc._28;
        // D s_37_1: write-var gs#18795 <= s_37_0
        fn_state.gs_18795 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_38_0: read-var gs#18795:u8
        let s_38_0: bool = fn_state.gs_18795;
        // N s_38_1: branch s_38_0 b64 b39
        if s_38_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#18796 <= s_39_0
        fn_state.gs_18796 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_40_0: read-var gs#18796:u8
        let s_40_0: bool = fn_state.gs_18796;
        // N s_40_1: branch s_40_0 b63 b41
        if s_40_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#18797 <= s_41_0
        fn_state.gs_18797 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_42_0: read-var gs#18797:u8
        let s_42_0: bool = fn_state.gs_18797;
        // N s_42_1: branch s_42_0 b59 b43
        if s_42_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_43_0: read-var accdesc.32:struct
        let s_43_0: bool = fn_state.accdesc._32;
        // N s_43_1: branch s_43_0 b58 b44
        if s_43_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#18798 <= s_44_0
        fn_state.gs_18798 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_45_0: read-var gs#18798:u8
        let s_45_0: bool = fn_state.gs_18798;
        // N s_45_1: branch s_45_0 b57 b46
        if s_45_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#18799 <= s_46_0
        fn_state.gs_18799 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_47_0: read-var gs#18799:u8
        let s_47_0: bool = fn_state.gs_18799;
        // N s_47_1: branch s_47_0 b56 b48
        if s_47_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#18800 <= s_48_0
        fn_state.gs_18800 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_49_0: read-var gs#18800:u8
        let s_49_0: bool = fn_state.gs_18800;
        // N s_49_1: branch s_49_0 b55 b50
        if s_49_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_51_0: read-var s2perms.3:struct
        let s_51_0: bool = fn_state.s2perms._3;
        // N s_51_1: branch s_51_0 b54 b52
        if s_51_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_52_0: read-var s2perms.13:struct
        let s_52_0: bool = fn_state.s2perms._13;
        // D s_52_1: read-var s2perms.15:struct
        let s_52_1: bool = fn_state.s2perms._15;
        // D s_52_2: read-var s2perms.14:struct
        let s_52_2: bool = fn_state.s2perms._14;
        // D s_52_3: cast zx s_52_1 -> bv
        let s_52_3: Bits = Bits::new(s_52_1 as u128, 1u16);
        // D s_52_4: cast zx s_52_2 -> bv
        let s_52_4: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_5: cast reint s_52_3 -> u128
        let s_52_5: u128 = (s_52_3.value() as u128);
        // D s_52_6: size-of s_52_3
        let s_52_6: u16 = s_52_3.length();
        // D s_52_7: cast reint s_52_4 -> u128
        let s_52_7: u128 = (s_52_4.value() as u128);
        // D s_52_8: size-of s_52_4
        let s_52_8: u16 = s_52_4.length();
        // D s_52_9: lsl s_52_5 s_52_8
        let s_52_9: u128 = s_52_5 << s_52_8;
        // D s_52_10: or s_52_9 s_52_7
        let s_52_10: u128 = ((s_52_9) | (s_52_7));
        // D s_52_11: add s_52_6 s_52_8
        let s_52_11: u16 = (s_52_6 + s_52_8);
        // D s_52_12: create-bits s_52_10 s_52_11
        let s_52_12: Bits = Bits::new(s_52_10, s_52_11);
        // D s_52_13: cast reint s_52_12 -> u8
        let s_52_13: u8 = (s_52_12.value() as u8);
        // D s_52_14: cast zx s_52_0 -> bv
        let s_52_14: Bits = Bits::new(s_52_0 as u128, 1u16);
        // D s_52_15: cast zx s_52_13 -> bv
        let s_52_15: Bits = Bits::new(s_52_13 as u128, 2u16);
        // D s_52_16: cast reint s_52_14 -> u128
        let s_52_16: u128 = (s_52_14.value() as u128);
        // D s_52_17: size-of s_52_14
        let s_52_17: u16 = s_52_14.length();
        // D s_52_18: cast reint s_52_15 -> u128
        let s_52_18: u128 = (s_52_15.value() as u128);
        // D s_52_19: size-of s_52_15
        let s_52_19: u16 = s_52_15.length();
        // D s_52_20: lsl s_52_16 s_52_19
        let s_52_20: u128 = s_52_16 << s_52_19;
        // D s_52_21: or s_52_20 s_52_18
        let s_52_21: u128 = ((s_52_20) | (s_52_18));
        // D s_52_22: add s_52_17 s_52_19
        let s_52_22: u16 = (s_52_17 + s_52_19);
        // D s_52_23: create-bits s_52_21 s_52_22
        let s_52_23: Bits = Bits::new(s_52_21, s_52_22);
        // D s_52_24: cast reint s_52_23 -> u8
        let s_52_24: u8 = (s_52_23.value() as u8);
        // D s_52_25: cast zx s_52_24 -> bv
        let s_52_25: Bits = Bits::new(s_52_24 as u128, 3u16);
        // C s_52_26: const #3u : u8
        let s_52_26: u8 = 3;
        // C s_52_27: cast zx s_52_26 -> bv
        let s_52_27: Bits = Bits::new(s_52_26 as u128, 3u16);
        // D s_52_28: cmp-eq s_52_25 s_52_27
        let s_52_28: bool = ((s_52_25) == (s_52_27));
        // D s_52_29: write-var mro <= s_52_28
        fn_state.mro = s_52_28;
        // N s_52_30: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_53_0: read-var fault:struct
        let s_53_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_53_1: read-var mro:u8
        let s_53_1: bool = fn_state.mro;
        // D s_53_2: create-product struct = ["s_53_0", "s_53_1"]
        let s_53_2: ProductType3b8bd97143a1dd5c = ProductType3b8bd97143a1dd5c {
            _0: s_53_0,
            _1: s_53_1,
        };
        // N s_53_3: return s_53_2
        return s_53_2;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_54_0: read-var s2perms.13:struct
        let s_54_0: bool = fn_state.s2perms._13;
        // D s_54_1: read-var s2perms.15:struct
        let s_54_1: bool = fn_state.s2perms._15;
        // D s_54_2: read-var s2perms.14:struct
        let s_54_2: bool = fn_state.s2perms._14;
        // D s_54_3: cast zx s_54_1 -> bv
        let s_54_3: Bits = Bits::new(s_54_1 as u128, 1u16);
        // D s_54_4: cast zx s_54_2 -> bv
        let s_54_4: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_5: cast reint s_54_3 -> u128
        let s_54_5: u128 = (s_54_3.value() as u128);
        // D s_54_6: size-of s_54_3
        let s_54_6: u16 = s_54_3.length();
        // D s_54_7: cast reint s_54_4 -> u128
        let s_54_7: u128 = (s_54_4.value() as u128);
        // D s_54_8: size-of s_54_4
        let s_54_8: u16 = s_54_4.length();
        // D s_54_9: lsl s_54_5 s_54_8
        let s_54_9: u128 = s_54_5 << s_54_8;
        // D s_54_10: or s_54_9 s_54_7
        let s_54_10: u128 = ((s_54_9) | (s_54_7));
        // D s_54_11: add s_54_6 s_54_8
        let s_54_11: u16 = (s_54_6 + s_54_8);
        // D s_54_12: create-bits s_54_10 s_54_11
        let s_54_12: Bits = Bits::new(s_54_10, s_54_11);
        // D s_54_13: cast reint s_54_12 -> u8
        let s_54_13: u8 = (s_54_12.value() as u8);
        // D s_54_14: cast zx s_54_0 -> bv
        let s_54_14: Bits = Bits::new(s_54_0 as u128, 1u16);
        // D s_54_15: cast zx s_54_13 -> bv
        let s_54_15: Bits = Bits::new(s_54_13 as u128, 2u16);
        // D s_54_16: cast reint s_54_14 -> u128
        let s_54_16: u128 = (s_54_14.value() as u128);
        // D s_54_17: size-of s_54_14
        let s_54_17: u16 = s_54_14.length();
        // D s_54_18: cast reint s_54_15 -> u128
        let s_54_18: u128 = (s_54_15.value() as u128);
        // D s_54_19: size-of s_54_15
        let s_54_19: u16 = s_54_15.length();
        // D s_54_20: lsl s_54_16 s_54_19
        let s_54_20: u128 = s_54_16 << s_54_19;
        // D s_54_21: or s_54_20 s_54_18
        let s_54_21: u128 = ((s_54_20) | (s_54_18));
        // D s_54_22: add s_54_17 s_54_19
        let s_54_22: u16 = (s_54_17 + s_54_19);
        // D s_54_23: create-bits s_54_21 s_54_22
        let s_54_23: Bits = Bits::new(s_54_21, s_54_22);
        // D s_54_24: cast reint s_54_23 -> u8
        let s_54_24: u8 = (s_54_23.value() as u8);
        // D s_54_25: read-var s2perms.4:struct
        let s_54_25: bool = fn_state.s2perms._4;
        // D s_54_26: read-var s2perms.6:struct
        let s_54_26: bool = fn_state.s2perms._6;
        // D s_54_27: read-var s2perms.5:struct
        let s_54_27: bool = fn_state.s2perms._5;
        // D s_54_28: cast zx s_54_26 -> bv
        let s_54_28: Bits = Bits::new(s_54_26 as u128, 1u16);
        // D s_54_29: cast zx s_54_27 -> bv
        let s_54_29: Bits = Bits::new(s_54_27 as u128, 1u16);
        // D s_54_30: cast reint s_54_28 -> u128
        let s_54_30: u128 = (s_54_28.value() as u128);
        // D s_54_31: size-of s_54_28
        let s_54_31: u16 = s_54_28.length();
        // D s_54_32: cast reint s_54_29 -> u128
        let s_54_32: u128 = (s_54_29.value() as u128);
        // D s_54_33: size-of s_54_29
        let s_54_33: u16 = s_54_29.length();
        // D s_54_34: lsl s_54_30 s_54_33
        let s_54_34: u128 = s_54_30 << s_54_33;
        // D s_54_35: or s_54_34 s_54_32
        let s_54_35: u128 = ((s_54_34) | (s_54_32));
        // D s_54_36: add s_54_31 s_54_33
        let s_54_36: u16 = (s_54_31 + s_54_33);
        // D s_54_37: create-bits s_54_35 s_54_36
        let s_54_37: Bits = Bits::new(s_54_35, s_54_36);
        // D s_54_38: cast reint s_54_37 -> u8
        let s_54_38: u8 = (s_54_37.value() as u8);
        // D s_54_39: cast zx s_54_25 -> bv
        let s_54_39: Bits = Bits::new(s_54_25 as u128, 1u16);
        // D s_54_40: cast zx s_54_38 -> bv
        let s_54_40: Bits = Bits::new(s_54_38 as u128, 2u16);
        // D s_54_41: cast reint s_54_39 -> u128
        let s_54_41: u128 = (s_54_39.value() as u128);
        // D s_54_42: size-of s_54_39
        let s_54_42: u16 = s_54_39.length();
        // D s_54_43: cast reint s_54_40 -> u128
        let s_54_43: u128 = (s_54_40.value() as u128);
        // D s_54_44: size-of s_54_40
        let s_54_44: u16 = s_54_40.length();
        // D s_54_45: lsl s_54_41 s_54_44
        let s_54_45: u128 = s_54_41 << s_54_44;
        // D s_54_46: or s_54_45 s_54_43
        let s_54_46: u128 = ((s_54_45) | (s_54_43));
        // D s_54_47: add s_54_42 s_54_44
        let s_54_47: u16 = (s_54_42 + s_54_44);
        // D s_54_48: create-bits s_54_46 s_54_47
        let s_54_48: Bits = Bits::new(s_54_46, s_54_47);
        // D s_54_49: cast reint s_54_48 -> u8
        let s_54_49: u8 = (s_54_48.value() as u8);
        // D s_54_50: cast zx s_54_24 -> bv
        let s_54_50: Bits = Bits::new(s_54_24 as u128, 3u16);
        // D s_54_51: cast zx s_54_49 -> bv
        let s_54_51: Bits = Bits::new(s_54_49 as u128, 3u16);
        // D s_54_52: and s_54_50 s_54_51
        let s_54_52: Bits = ((s_54_50) & (s_54_51));
        // D s_54_53: cast reint s_54_52 -> u8
        let s_54_53: u8 = (s_54_52.value() as u8);
        // D s_54_54: cast zx s_54_53 -> bv
        let s_54_54: Bits = Bits::new(s_54_53 as u128, 3u16);
        // C s_54_55: const #3u : u8
        let s_54_55: u8 = 3;
        // C s_54_56: cast zx s_54_55 -> bv
        let s_54_56: Bits = Bits::new(s_54_55 as u128, 3u16);
        // D s_54_57: cmp-eq s_54_54 s_54_56
        let s_54_57: bool = ((s_54_54) == (s_54_56));
        // D s_54_58: write-var mro <= s_54_57
        fn_state.mro = s_54_57;
        // N s_54_59: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_55_0: const #5u : u32
        let s_55_0: u32 = 5;
        // D s_55_1: write-var fault.16 <= s_55_0
        fn_state.fault._16 = s_55_0;
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // D s_55_3: write-var fault.3 <= s_55_2
        fn_state.fault._3 = s_55_2;
        // C s_55_4: const #1u : u8
        let s_55_4: bool = true;
        // D s_55_5: write-var fault.19 <= s_55_4
        fn_state.fault._19 = s_55_4;
        // N s_55_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_56_0: read-var permissions.8:struct
        let s_56_0: bool = fn_state.permissions._8;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #0u : u8
        let s_56_2: bool = false;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#18800 <= s_56_4
        fn_state.gs_18800 = s_56_4;
        // N s_56_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_57_0: read-var walkparams.17:struct
        let s_57_0: bool = fn_state.walkparams._17;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #1u : u8
        let s_57_2: bool = true;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#18799 <= s_57_4
        fn_state.gs_18799 = s_57_4;
        // N s_57_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_58_0: read-var walkparams.7:struct
        let s_58_0: bool = fn_state.walkparams._7;
        // D s_58_1: read-var walkparams.9:struct
        let s_58_1: bool = fn_state.walkparams._9;
        // D s_58_2: cast zx s_58_0 -> bv
        let s_58_2: Bits = Bits::new(s_58_0 as u128, 1u16);
        // D s_58_3: cast zx s_58_1 -> bv
        let s_58_3: Bits = Bits::new(s_58_1 as u128, 1u16);
        // D s_58_4: cast reint s_58_2 -> u128
        let s_58_4: u128 = (s_58_2.value() as u128);
        // D s_58_5: size-of s_58_2
        let s_58_5: u16 = s_58_2.length();
        // D s_58_6: cast reint s_58_3 -> u128
        let s_58_6: u128 = (s_58_3.value() as u128);
        // D s_58_7: size-of s_58_3
        let s_58_7: u16 = s_58_3.length();
        // D s_58_8: lsl s_58_4 s_58_7
        let s_58_8: u128 = s_58_4 << s_58_7;
        // D s_58_9: or s_58_8 s_58_6
        let s_58_9: u128 = ((s_58_8) | (s_58_6));
        // D s_58_10: add s_58_5 s_58_7
        let s_58_10: u16 = (s_58_5 + s_58_7);
        // D s_58_11: create-bits s_58_9 s_58_10
        let s_58_11: Bits = Bits::new(s_58_9, s_58_10);
        // D s_58_12: cast reint s_58_11 -> u8
        let s_58_12: u8 = (s_58_11.value() as u8);
        // D s_58_13: cast zx s_58_12 -> bv
        let s_58_13: Bits = Bits::new(s_58_12 as u128, 2u16);
        // C s_58_14: const #3u : u8
        let s_58_14: u8 = 3;
        // C s_58_15: cast zx s_58_14 -> bv
        let s_58_15: Bits = Bits::new(s_58_14 as u128, 2u16);
        // D s_58_16: cmp-eq s_58_13 s_58_15
        let s_58_16: bool = ((s_58_13) == (s_58_15));
        // D s_58_17: not s_58_16
        let s_58_17: bool = !s_58_16;
        // D s_58_18: write-var gs#18798 <= s_58_17
        fn_state.gs_18798 = s_58_17;
        // N s_58_19: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_59_0: const #5u : u32
        let s_59_0: u32 = 5;
        // D s_59_1: write-var fault.16 <= s_59_0
        fn_state.fault._16 = s_59_0;
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // D s_59_3: write-var fault.17 <= s_59_2
        fn_state.fault._17 = s_59_2;
        // D s_59_4: read-var accdesc.27:struct
        let s_59_4: bool = fn_state.accdesc._27;
        // N s_59_5: branch s_59_4 b62 b60
        if s_59_4 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#18805 <= s_60_0
        fn_state.gs_18805 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_61_0: read-var gs#18805:u8
        let s_61_0: bool = fn_state.gs_18805;
        // D s_61_1: write-var fault.19 <= s_61_0
        fn_state.fault._19 = s_61_0;
        // N s_61_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_62_0: read-var accdesc.32:struct
        let s_62_0: bool = fn_state.accdesc._32;
        // D s_62_1: write-var gs#18805 <= s_62_0
        fn_state.gs_18805 = s_62_0;
        // N s_62_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_63_0: read-var permissions.11:struct
        let s_63_0: bool = fn_state.permissions._11;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#18797 <= s_63_4
        fn_state.gs_18797 = s_63_4;
        // N s_63_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_64_0: read-var ipa.2:struct
        let s_64_0: ProductTypef170cab34335b70c = fn_state.ipa._2;
        // D s_64_1: write-var ga#14077 <= s_64_0
        fn_state.ga_14077 = s_64_0;
        // D s_64_2: read-var ga#14077.6:struct
        let s_64_2: u32 = fn_state.ga_14077._6;
        // C s_64_3: const #1u : u32
        let s_64_3: u32 = 1;
        // D s_64_4: cmp-eq s_64_2 s_64_3
        let s_64_4: bool = ((s_64_2) == (s_64_3));
        // D s_64_5: write-var gs#18796 <= s_64_4
        fn_state.gs_18796 = s_64_4;
        // N s_64_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#18795 <= s_65_0
        fn_state.gs_18795 = s_65_0;
        // N s_65_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_66_0: const #5u : u32
        let s_66_0: u32 = 5;
        // D s_66_1: write-var fault.16 <= s_66_0
        fn_state.fault._16 = s_66_0;
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // D s_66_3: write-var fault.19 <= s_66_2
        fn_state.fault._19 = s_66_2;
        // N s_66_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_67_0: read-var w:u8
        let s_67_0: bool = fn_state.w;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#18794 <= s_67_4
        fn_state.gs_18794 = s_67_4;
        // N s_67_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_68_0: const #5u : u32
        let s_68_0: u32 = 5;
        // D s_68_1: write-var fault.16 <= s_68_0
        fn_state.fault._16 = s_68_0;
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // D s_68_3: write-var fault.19 <= s_68_2
        fn_state.fault._19 = s_68_2;
        // N s_68_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_69_0: read-var r:u8
        let s_69_0: bool = fn_state.r;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #0u : u8
        let s_69_2: bool = false;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#18793 <= s_69_4
        fn_state.gs_18793 = s_69_4;
        // N s_69_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_70_0: const #5u : u32
        let s_70_0: u32 = 5;
        // D s_70_1: write-var fault.16 <= s_70_0
        fn_state.fault._16 = s_70_0;
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // D s_70_3: write-var fault.11 <= s_70_2
        fn_state.fault._11 = s_70_2;
        // C s_70_4: const #1u : u8
        let s_70_4: bool = true;
        // D s_70_5: write-var fault.19 <= s_70_4
        fn_state.fault._19 = s_70_4;
        // N s_70_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_71_0: read-var ow:u8
        let s_71_0: bool = fn_state.ow;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#18792 <= s_71_4
        fn_state.gs_18792 = s_71_4;
        // N s_71_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_72_0: read-var s2perms.3:struct
        let s_72_0: bool = fn_state.s2perms._3;
        // D s_72_1: write-var gs#18791 <= s_72_0
        fn_state.gs_18791 = s_72_0;
        // N s_72_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_73_0: const #5u : u32
        let s_73_0: u32 = 5;
        // D s_73_1: write-var fault.16 <= s_73_0
        fn_state.fault._16 = s_73_0;
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // D s_73_3: write-var fault.11 <= s_73_2
        fn_state.fault._11 = s_73_2;
        // C s_73_4: const #0u : u8
        let s_73_4: bool = false;
        // D s_73_5: write-var fault.19 <= s_73_4
        fn_state.fault._19 = s_73_4;
        // N s_73_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_74_0: read-var or:u8
        let s_74_0: bool = fn_state.or;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#18790 <= s_74_4
        fn_state.gs_18790 = s_74_4;
        // N s_74_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_75_0: read-var s2perms.3:struct
        let s_75_0: bool = fn_state.s2perms._3;
        // D s_75_1: write-var gs#18789 <= s_75_0
        fn_state.gs_18789 = s_75_0;
        // N s_75_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_76_0: const #440u : u32
        let s_76_0: u32 = 440;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call ELUsingAArch32(s_76_1)
        let s_76_2: bool = ELUsingAArch32(state, tracer, s_76_1);
        // D s_76_3: not s_76_2
        let s_76_3: bool = !s_76_2;
        // N s_76_4: branch s_76_3 b114 b77
        if s_76_3 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#18812 <= s_77_0
        fn_state.gs_18812 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_78_0: read-var gs#18812:u8
        let s_78_0: bool = fn_state.gs_18812;
        // N s_78_1: branch s_78_0 b113 b79
        if s_78_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#18813 <= s_79_0
        fn_state.gs_18813 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_80_0: read-var gs#18813:u8
        let s_80_0: bool = fn_state.gs_18813;
        // N s_80_1: branch s_80_0 b112 b81
        if s_80_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#18814 <= s_81_0
        fn_state.gs_18814 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_82_0: read-var gs#18814:u8
        let s_82_0: bool = fn_state.gs_18814;
        // N s_82_1: branch s_82_0 b111 b83
        if s_82_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#18815 <= s_83_0
        fn_state.gs_18815 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_84_0: read-var gs#18815:u8
        let s_84_0: bool = fn_state.gs_18815;
        // N s_84_1: branch s_84_0 b110 b85
        if s_84_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_85_0: read-var walkparams.1:struct
        let s_85_0: bool = fn_state.walkparams._1;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // N s_85_5: branch s_85_4 b109 b86
        if s_85_4 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#18816 <= s_86_0
        fn_state.gs_18816 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_87_0: read-var gs#18816:u8
        let s_87_0: bool = fn_state.gs_18816;
        // N s_87_1: branch s_87_0 b108 b88
        if s_87_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#18817 <= s_88_0
        fn_state.gs_18817 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_89_0: read-var gs#18817:u8
        let s_89_0: bool = fn_state.gs_18817;
        // N s_89_1: branch s_89_0 b107 b90
        if s_89_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_90_0: const #440u : u32
        let s_90_0: u32 = 440;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // N s_90_4: branch s_90_3 b106 b91
        if s_90_3 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#18818 <= s_91_0
        fn_state.gs_18818 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_92_0: read-var gs#18818:u8
        let s_92_0: bool = fn_state.gs_18818;
        // N s_92_1: branch s_92_0 b105 b93
        if s_92_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#18819 <= s_93_0
        fn_state.gs_18819 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_94_0: read-var gs#18819:u8
        let s_94_0: bool = fn_state.gs_18819;
        // N s_94_1: branch s_94_0 b104 b95
        if s_94_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#18820 <= s_95_0
        fn_state.gs_18820 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_96_0: read-var gs#18820:u8
        let s_96_0: bool = fn_state.gs_18820;
        // N s_96_1: branch s_96_0 b103 b97
        if s_96_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_97_0: read-var walkparams.1:struct
        let s_97_0: bool = fn_state.walkparams._1;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // N s_97_5: branch s_97_4 b102 b98
        if s_97_4 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#18821 <= s_98_0
        fn_state.gs_18821 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_99_0: read-var gs#18821:u8
        let s_99_0: bool = fn_state.gs_18821;
        // N s_99_1: branch s_99_0 b101 b100
        if s_99_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_100_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_101_0: const #5u : u32
        let s_101_0: u32 = 5;
        // D s_101_1: write-var fault.16 <= s_101_0
        fn_state.fault._16 = s_101_0;
        // N s_101_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_102_0: read-var w:u8
        let s_102_0: bool = fn_state.w;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #0u : u8
        let s_102_2: bool = false;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#18821 <= s_102_4
        fn_state.gs_18821 = s_102_4;
        // N s_102_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_103_0: const #5u : u32
        let s_103_0: u32 = 5;
        // D s_103_1: write-var fault.16 <= s_103_0
        fn_state.fault._16 = s_103_0;
        // N s_103_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_104_0: const #"Permission fault on EL0 IC_IVAU execution" : str
        let s_104_0: &'static str = "Permission fault on EL0 IC_IVAU execution";
        // S s_104_1: call __IMPDEF_boolean(s_104_0)
        let s_104_1: bool = u__IMPDEF_boolean(state, tracer, s_104_0);
        // D s_104_2: write-var gs#18820 <= s_104_1
        fn_state.gs_18820 = s_104_1;
        // N s_104_3: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_105_0: read-var r:u8
        let s_105_0: bool = fn_state.r;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #0u : u8
        let s_105_2: bool = false;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#18819 <= s_105_4
        fn_state.gs_18819 = s_105_4;
        // N s_105_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_106_0: read-var accdesc.8:struct
        let s_106_0: u8 = fn_state.accdesc._8;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 2u16);
        // C s_106_2: const #448u : u32
        let s_106_2: u32 = 448;
        // D s_106_3: read-reg s_106_2:u8
        let s_106_3: u8 = {
            let value = state.read_register::<u8>(s_106_2 as isize);
            tracer.read_register(s_106_2 as isize, value);
            value
        };
        // D s_106_4: cast zx s_106_3 -> bv
        let s_106_4: Bits = Bits::new(s_106_3 as u128, 2u16);
        // D s_106_5: cmp-eq s_106_1 s_106_4
        let s_106_5: bool = ((s_106_1) == (s_106_4));
        // D s_106_6: write-var gs#18818 <= s_106_5
        fn_state.gs_18818 = s_106_5;
        // N s_106_7: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_107_0: const #5u : u32
        let s_107_0: u32 = 5;
        // D s_107_1: write-var fault.16 <= s_107_0
        fn_state.fault._16 = s_107_0;
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // D s_107_3: write-var fault.11 <= s_107_2
        fn_state.fault._11 = s_107_2;
        // N s_107_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_108_0: read-var ow:u8
        let s_108_0: bool = fn_state.ow;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #0u : u8
        let s_108_2: bool = false;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#18817 <= s_108_4
        fn_state.gs_18817 = s_108_4;
        // N s_108_6: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_109_0: read-var s2perms.3:struct
        let s_109_0: bool = fn_state.s2perms._3;
        // D s_109_1: write-var gs#18816 <= s_109_0
        fn_state.gs_18816 = s_109_0;
        // N s_109_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_110_0: const #5u : u32
        let s_110_0: u32 = 5;
        // D s_110_1: write-var fault.16 <= s_110_0
        fn_state.fault._16 = s_110_0;
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // D s_110_3: write-var fault.11 <= s_110_2
        fn_state.fault._11 = s_110_2;
        // N s_110_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_111_0: const #"Permission fault on EL0 IC_IVAU execution" : str
        let s_111_0: &'static str = "Permission fault on EL0 IC_IVAU execution";
        // S s_111_1: call __IMPDEF_boolean(s_111_0)
        let s_111_1: bool = u__IMPDEF_boolean(state, tracer, s_111_0);
        // D s_111_2: write-var gs#18815 <= s_111_1
        fn_state.gs_18815 = s_111_1;
        // N s_111_3: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_112_0: read-var or:u8
        let s_112_0: bool = fn_state.or;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #0u : u8
        let s_112_2: bool = false;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#18814 <= s_112_4
        fn_state.gs_18814 = s_112_4;
        // N s_112_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_113_0: read-var s2perms.3:struct
        let s_113_0: bool = fn_state.s2perms._3;
        // D s_113_1: write-var gs#18813 <= s_113_0
        fn_state.gs_18813 = s_113_0;
        // N s_113_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_114_0: read-var accdesc.8:struct
        let s_114_0: u8 = fn_state.accdesc._8;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 2u16);
        // C s_114_2: const #448u : u32
        let s_114_2: u32 = 448;
        // D s_114_3: read-reg s_114_2:u8
        let s_114_3: u8 = {
            let value = state.read_register::<u8>(s_114_2 as isize);
            tracer.read_register(s_114_2 as isize, value);
            value
        };
        // D s_114_4: cast zx s_114_3 -> bv
        let s_114_4: Bits = Bits::new(s_114_3 as u128, 2u16);
        // D s_114_5: cmp-eq s_114_1 s_114_4
        let s_114_5: bool = ((s_114_1) == (s_114_4));
        // D s_114_6: write-var gs#18812 <= s_114_5
        fn_state.gs_18812 = s_114_5;
        // N s_114_7: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_115_0: read-var accdesc.5:struct
        let s_115_0: u32 = fn_state.accdesc._5;
        // C s_115_1: const #1u : u32
        let s_115_1: u32 = 1;
        // D s_115_2: cmp-eq s_115_0 s_115_1
        let s_115_2: bool = ((s_115_0) == (s_115_1));
        // N s_115_3: branch s_115_2 b161 b116
        if s_115_2 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_116_0: const #440u : u32
        let s_116_0: u32 = 440;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call ELUsingAArch32(s_116_1)
        let s_116_2: bool = ELUsingAArch32(state, tracer, s_116_1);
        // D s_116_3: not s_116_2
        let s_116_3: bool = !s_116_2;
        // N s_116_4: branch s_116_3 b160 b117
        if s_116_3 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#18824 <= s_117_0
        fn_state.gs_18824 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_118_0: read-var gs#18824:u8
        let s_118_0: bool = fn_state.gs_18824;
        // N s_118_1: branch s_118_0 b159 b119
        if s_118_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#18825 <= s_119_0
        fn_state.gs_18825 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_120_0: read-var gs#18825:u8
        let s_120_0: bool = fn_state.gs_18825;
        // N s_120_1: branch s_120_0 b158 b121
        if s_120_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#18826 <= s_121_0
        fn_state.gs_18826 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_122_0: read-var gs#18826:u8
        let s_122_0: bool = fn_state.gs_18826;
        // N s_122_1: branch s_122_0 b157 b123
        if s_122_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_123_0: read-var walkparams.1:struct
        let s_123_0: bool = fn_state.walkparams._1;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // N s_123_5: branch s_123_4 b156 b124
        if s_123_4 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#18827 <= s_124_0
        fn_state.gs_18827 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_125_0: read-var gs#18827:u8
        let s_125_0: bool = fn_state.gs_18827;
        // N s_125_1: branch s_125_0 b155 b126
        if s_125_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_126_0: const #0u : u8
        let s_126_0: bool = false;
        // D s_126_1: write-var gs#18828 <= s_126_0
        fn_state.gs_18828 = s_126_0;
        // N s_126_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_127_0: read-var gs#18828:u8
        let s_127_0: bool = fn_state.gs_18828;
        // N s_127_1: branch s_127_0 b154 b128
        if s_127_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#18829 <= s_128_0
        fn_state.gs_18829 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_129_0: read-var gs#18829:u8
        let s_129_0: bool = fn_state.gs_18829;
        // N s_129_1: branch s_129_0 b153 b130
        if s_129_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#18830 <= s_130_0
        fn_state.gs_18830 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_131_0: read-var gs#18830:u8
        let s_131_0: bool = fn_state.gs_18830;
        // N s_131_1: branch s_131_0 b152 b132
        if s_131_0 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_132_0: const #440u : u32
        let s_132_0: u32 = 440;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call ELUsingAArch32(s_132_1)
        let s_132_2: bool = ELUsingAArch32(state, tracer, s_132_1);
        // D s_132_3: not s_132_2
        let s_132_3: bool = !s_132_2;
        // N s_132_4: branch s_132_3 b151 b133
        if s_132_3 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#18831 <= s_133_0
        fn_state.gs_18831 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_134_0: read-var gs#18831:u8
        let s_134_0: bool = fn_state.gs_18831;
        // N s_134_1: branch s_134_0 b150 b135
        if s_134_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#18832 <= s_135_0
        fn_state.gs_18832 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_136_0: read-var gs#18832:u8
        let s_136_0: bool = fn_state.gs_18832;
        // N s_136_1: branch s_136_0 b149 b137
        if s_136_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_137_0: read-var walkparams.1:struct
        let s_137_0: bool = fn_state.walkparams._1;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // N s_137_5: branch s_137_4 b148 b138
        if s_137_4 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#18833 <= s_138_0
        fn_state.gs_18833 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_139_0: read-var gs#18833:u8
        let s_139_0: bool = fn_state.gs_18833;
        // N s_139_1: branch s_139_0 b147 b140
        if s_139_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#18834 <= s_140_0
        fn_state.gs_18834 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_141_0: read-var gs#18834:u8
        let s_141_0: bool = fn_state.gs_18834;
        // N s_141_1: branch s_141_0 b146 b142
        if s_141_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#18835 <= s_142_0
        fn_state.gs_18835 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_143_0: read-var gs#18835:u8
        let s_143_0: bool = fn_state.gs_18835;
        // N s_143_1: branch s_143_0 b145 b144
        if s_143_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_144_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_145_0: const #5u : u32
        let s_145_0: u32 = 5;
        // D s_145_1: write-var fault.16 <= s_145_0
        fn_state.fault._16 = s_145_0;
        // N s_145_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_146_0: read-var w:u8
        let s_146_0: bool = fn_state.w;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #0u : u8
        let s_146_2: bool = false;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#18835 <= s_146_4
        fn_state.gs_18835 = s_146_4;
        // N s_146_6: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_147_0: read-var accdesc.5:struct
        let s_147_0: u32 = fn_state.accdesc._5;
        // C s_147_1: const #2u : u32
        let s_147_1: u32 = 2;
        // D s_147_2: cmp-eq s_147_0 s_147_1
        let s_147_2: bool = ((s_147_0) == (s_147_1));
        // D s_147_3: write-var gs#18834 <= s_147_2
        fn_state.gs_18834 = s_147_2;
        // N s_147_4: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_148_0: read-var accdesc.19:struct
        let s_148_0: u32 = fn_state.accdesc._19;
        // C s_148_1: const #2u : u32
        let s_148_1: u32 = 2;
        // D s_148_2: cmp-eq s_148_0 s_148_1
        let s_148_2: bool = ((s_148_0) == (s_148_1));
        // D s_148_3: write-var gs#18833 <= s_148_2
        fn_state.gs_18833 = s_148_2;
        // N s_148_4: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_149_0: const #5u : u32
        let s_149_0: u32 = 5;
        // D s_149_1: write-var fault.16 <= s_149_0
        fn_state.fault._16 = s_149_0;
        // N s_149_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_150_0: read-var r:u8
        let s_150_0: bool = fn_state.r;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 1u16);
        // C s_150_2: const #0u : u8
        let s_150_2: bool = false;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 1u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // D s_150_5: write-var gs#18832 <= s_150_4
        fn_state.gs_18832 = s_150_4;
        // N s_150_6: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_151_0: read-var accdesc.8:struct
        let s_151_0: u8 = fn_state.accdesc._8;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 2u16);
        // C s_151_2: const #448u : u32
        let s_151_2: u32 = 448;
        // D s_151_3: read-reg s_151_2:u8
        let s_151_3: u8 = {
            let value = state.read_register::<u8>(s_151_2 as isize);
            tracer.read_register(s_151_2 as isize, value);
            value
        };
        // D s_151_4: cast zx s_151_3 -> bv
        let s_151_4: Bits = Bits::new(s_151_3 as u128, 2u16);
        // D s_151_5: cmp-eq s_151_1 s_151_4
        let s_151_5: bool = ((s_151_1) == (s_151_4));
        // D s_151_6: write-var gs#18831 <= s_151_5
        fn_state.gs_18831 = s_151_5;
        // N s_151_7: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_152_0: const #5u : u32
        let s_152_0: u32 = 5;
        // D s_152_1: write-var fault.16 <= s_152_0
        fn_state.fault._16 = s_152_0;
        // C s_152_2: const #1u : u8
        let s_152_2: bool = true;
        // D s_152_3: write-var fault.11 <= s_152_2
        fn_state.fault._11 = s_152_2;
        // N s_152_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_153_0: read-var ow:u8
        let s_153_0: bool = fn_state.ow;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 1u16);
        // C s_153_2: const #0u : u8
        let s_153_2: bool = false;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 1u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // D s_153_5: write-var gs#18830 <= s_153_4
        fn_state.gs_18830 = s_153_4;
        // N s_153_6: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_154_0: read-var s2perms.3:struct
        let s_154_0: bool = fn_state.s2perms._3;
        // D s_154_1: write-var gs#18829 <= s_154_0
        fn_state.gs_18829 = s_154_0;
        // N s_154_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_155_0: read-var accdesc.5:struct
        let s_155_0: u32 = fn_state.accdesc._5;
        // C s_155_1: const #2u : u32
        let s_155_1: u32 = 2;
        // D s_155_2: cmp-eq s_155_0 s_155_1
        let s_155_2: bool = ((s_155_0) == (s_155_1));
        // D s_155_3: write-var gs#18828 <= s_155_2
        fn_state.gs_18828 = s_155_2;
        // N s_155_4: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_156_0: read-var accdesc.19:struct
        let s_156_0: u32 = fn_state.accdesc._19;
        // C s_156_1: const #2u : u32
        let s_156_1: u32 = 2;
        // D s_156_2: cmp-eq s_156_0 s_156_1
        let s_156_2: bool = ((s_156_0) == (s_156_1));
        // D s_156_3: write-var gs#18827 <= s_156_2
        fn_state.gs_18827 = s_156_2;
        // N s_156_4: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_157_0: const #5u : u32
        let s_157_0: u32 = 5;
        // D s_157_1: write-var fault.16 <= s_157_0
        fn_state.fault._16 = s_157_0;
        // C s_157_2: const #1u : u8
        let s_157_2: bool = true;
        // D s_157_3: write-var fault.11 <= s_157_2
        fn_state.fault._11 = s_157_2;
        // N s_157_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_158_0: read-var or:u8
        let s_158_0: bool = fn_state.or;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #0u : u8
        let s_158_2: bool = false;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#18826 <= s_158_4
        fn_state.gs_18826 = s_158_4;
        // N s_158_6: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_159_0: read-var s2perms.3:struct
        let s_159_0: bool = fn_state.s2perms._3;
        // D s_159_1: write-var gs#18825 <= s_159_0
        fn_state.gs_18825 = s_159_0;
        // N s_159_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_160_0: read-var accdesc.8:struct
        let s_160_0: u8 = fn_state.accdesc._8;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 2u16);
        // C s_160_2: const #448u : u32
        let s_160_2: u32 = 448;
        // D s_160_3: read-reg s_160_2:u8
        let s_160_3: u8 = {
            let value = state.read_register::<u8>(s_160_2 as isize);
            tracer.read_register(s_160_2 as isize, value);
            value
        };
        // D s_160_4: cast zx s_160_3 -> bv
        let s_160_4: Bits = Bits::new(s_160_3 as u128, 2u16);
        // D s_160_5: cmp-eq s_160_1 s_160_4
        let s_160_5: bool = ((s_160_1) == (s_160_4));
        // D s_160_6: write-var gs#18824 <= s_160_5
        fn_state.gs_18824 = s_160_5;
        // N s_160_7: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_161_0: const #440u : u32
        let s_161_0: u32 = 440;
        // D s_161_1: read-reg s_161_0:u8
        let s_161_1: u8 = {
            let value = state.read_register::<u8>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // D s_161_2: call ELUsingAArch32(s_161_1)
        let s_161_2: bool = ELUsingAArch32(state, tracer, s_161_1);
        // D s_161_3: not s_161_2
        let s_161_3: bool = !s_161_2;
        // N s_161_4: branch s_161_3 b176 b162
        if s_161_3 {
            return block_176(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_162_0: const #0u : u8
        let s_162_0: bool = false;
        // D s_162_1: write-var gs#18838 <= s_162_0
        fn_state.gs_18838 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_163_0: read-var gs#18838:u8
        let s_163_0: bool = fn_state.gs_18838;
        // N s_163_1: branch s_163_0 b175 b164
        if s_163_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#18839 <= s_164_0
        fn_state.gs_18839 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_165_0: read-var gs#18839:u8
        let s_165_0: bool = fn_state.gs_18839;
        // N s_165_1: branch s_165_0 b174 b166
        if s_165_0 {
            return block_174(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_166_0: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_167_0: const #440u : u32
        let s_167_0: u32 = 440;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // D s_167_2: call ELUsingAArch32(s_167_1)
        let s_167_2: bool = ELUsingAArch32(state, tracer, s_167_1);
        // D s_167_3: not s_167_2
        let s_167_3: bool = !s_167_2;
        // N s_167_4: branch s_167_3 b173 b168
        if s_167_3 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#18840 <= s_168_0
        fn_state.gs_18840 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_169_0: read-var gs#18840:u8
        let s_169_0: bool = fn_state.gs_18840;
        // N s_169_1: branch s_169_0 b172 b170
        if s_169_0 {
            return block_172(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_170_0: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_171_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_172_0: const #5u : u32
        let s_172_0: u32 = 5;
        // D s_172_1: write-var fault.16 <= s_172_0
        fn_state.fault._16 = s_172_0;
        // N s_172_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_173_0: read-var w:u8
        let s_173_0: bool = fn_state.w;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 1u16);
        // C s_173_2: const #0u : u8
        let s_173_2: bool = false;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 1u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#18840 <= s_173_4
        fn_state.gs_18840 = s_173_4;
        // N s_173_6: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_174_0: const #5u : u32
        let s_174_0: u32 = 5;
        // D s_174_1: write-var fault.16 <= s_174_0
        fn_state.fault._16 = s_174_0;
        // C s_174_2: const #1u : u8
        let s_174_2: bool = true;
        // D s_174_3: write-var fault.11 <= s_174_2
        fn_state.fault._11 = s_174_2;
        // N s_174_4: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_175_0: read-var ow:u8
        let s_175_0: bool = fn_state.ow;
        // D s_175_1: cast zx s_175_0 -> bv
        let s_175_1: Bits = Bits::new(s_175_0 as u128, 1u16);
        // C s_175_2: const #0u : u8
        let s_175_2: bool = false;
        // C s_175_3: cast zx s_175_2 -> bv
        let s_175_3: Bits = Bits::new(s_175_2 as u128, 1u16);
        // D s_175_4: cmp-eq s_175_1 s_175_3
        let s_175_4: bool = ((s_175_1) == (s_175_3));
        // D s_175_5: write-var gs#18839 <= s_175_4
        fn_state.gs_18839 = s_175_4;
        // N s_175_6: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_176_0: read-var s2perms.3:struct
        let s_176_0: bool = fn_state.s2perms._3;
        // D s_176_1: write-var gs#18838 <= s_176_0
        fn_state.gs_18838 = s_176_0;
        // N s_176_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_177_0: read-var s2perms.3:struct
        let s_177_0: bool = fn_state.s2perms._3;
        // N s_177_1: branch s_177_0 b194 b178
        if s_177_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_178_0: const #0u : u8
        let s_178_0: bool = false;
        // D s_178_1: write-var gs#18844 <= s_178_0
        fn_state.gs_18844 = s_178_0;
        // N s_178_2: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_179_0: read-var gs#18844:u8
        let s_179_0: bool = fn_state.gs_18844;
        // N s_179_1: branch s_179_0 b193 b180
        if s_179_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_180_0: read-var memtype:u32
        let s_180_0: u32 = fn_state.memtype;
        // C s_180_1: const #1u : u32
        let s_180_1: u32 = 1;
        // D s_180_2: cmp-eq s_180_0 s_180_1
        let s_180_2: bool = ((s_180_0) == (s_180_1));
        // N s_180_3: branch s_180_2 b192 b181
        if s_180_2 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_181_0: const #0u : u8
        let s_181_0: bool = false;
        // D s_181_1: write-var gs#18845 <= s_181_0
        fn_state.gs_18845 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_182_0: read-var gs#18845:u8
        let s_182_0: bool = fn_state.gs_18845;
        // N s_182_1: branch s_182_0 b191 b183
        if s_182_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_183_0: read-var accdesc.25:struct
        let s_183_0: u32 = fn_state.accdesc._25;
        // C s_183_1: const #2u : u32
        let s_183_1: u32 = 2;
        // D s_183_2: cmp-eq s_183_0 s_183_1
        let s_183_2: bool = ((s_183_0) == (s_183_1));
        // N s_183_3: branch s_183_2 b190 b184
        if s_183_2 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_184(state, tracer, fn_state);
        };
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_184_0: const #0u : u8
        let s_184_0: bool = false;
        // D s_184_1: write-var gs#18846 <= s_184_0
        fn_state.gs_18846 = s_184_0;
        // N s_184_2: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_185_0: read-var gs#18846:u8
        let s_185_0: bool = fn_state.gs_18846;
        // N s_185_1: branch s_185_0 b189 b186
        if s_185_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_186_0: read-var s2perms.16:struct
        let s_186_0: bool = fn_state.s2perms._16;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #0u : u8
        let s_186_2: bool = false;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // N s_186_5: branch s_186_4 b188 b187
        if s_186_4 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_187_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_188_0: const #5u : u32
        let s_188_0: u32 = 5;
        // D s_188_1: write-var fault.16 <= s_188_0
        fn_state.fault._16 = s_188_0;
        // N s_188_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_189_0: const #5u : u32
        let s_189_0: u32 = 5;
        // D s_189_1: write-var fault.16 <= s_189_0
        fn_state.fault._16 = s_189_0;
        // N s_189_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_190_0: read-var walkstate.0:struct
        let s_190_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_190_1: write-var ga#14001 <= s_190_0
        fn_state.ga_14001 = s_190_0;
        // D s_190_2: read-var ga#14001.1:struct
        let s_190_2: u32 = fn_state.ga_14001._1;
        // C s_190_3: const #3u : u32
        let s_190_3: u32 = 3;
        // D s_190_4: cmp-eq s_190_2 s_190_3
        let s_190_4: bool = ((s_190_2) == (s_190_3));
        // D s_190_5: write-var gs#18846 <= s_190_4
        fn_state.gs_18846 = s_190_4;
        // N s_190_6: jump b185
        return block_185(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_191_0: const #5u : u32
        let s_191_0: u32 = 5;
        // D s_191_1: write-var fault.16 <= s_191_0
        fn_state.fault._16 = s_191_0;
        // N s_191_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_192_0: const #7u : u32
        let s_192_0: u32 = 7;
        // S s_192_1: call ConstrainUnpredictable(s_192_0)
        let s_192_1: u32 = ConstrainUnpredictable(state, tracer, s_192_0);
        // C s_192_2: const #12u : u32
        let s_192_2: u32 = 12;
        // S s_192_3: cmp-eq s_192_1 s_192_2
        let s_192_3: bool = ((s_192_1) == (s_192_2));
        // D s_192_4: write-var gs#18845 <= s_192_3
        fn_state.gs_18845 = s_192_3;
        // N s_192_5: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_193_0: const #5u : u32
        let s_193_0: u32 = 5;
        // D s_193_1: write-var fault.16 <= s_193_0
        fn_state.fault._16 = s_193_0;
        // C s_193_2: const #1u : u8
        let s_193_2: bool = true;
        // D s_193_3: write-var fault.11 <= s_193_2
        fn_state.fault._11 = s_193_2;
        // N s_193_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_194_0: read-var s2perms.7:struct
        let s_194_0: bool = fn_state.s2perms._7;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #0u : u8
        let s_194_2: bool = false;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#18844 <= s_194_4
        fn_state.gs_18844 = s_194_4;
        // N s_194_6: jump b179
        return block_179(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_195_0: const #5u : u32
        let s_195_0: u32 = 5;
        // D s_195_1: write-var fault.16 <= s_195_0
        fn_state.fault._16 = s_195_0;
        // C s_195_2: const #1u : u8
        let s_195_2: bool = true;
        // D s_195_3: write-var fault.1 <= s_195_2
        fn_state.fault._1 = s_195_2;
        // N s_195_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_196_0: read-var accdesc.8:struct
        let s_196_0: u8 = fn_state.accdesc._8;
        // D s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 2u16);
        // C s_196_2: const #448u : u32
        let s_196_2: u32 = 448;
        // D s_196_3: read-reg s_196_2:u8
        let s_196_3: u8 = {
            let value = state.read_register::<u8>(s_196_2 as isize);
            tracer.read_register(s_196_2 as isize, value);
            value
        };
        // D s_196_4: cast zx s_196_3 -> bv
        let s_196_4: Bits = Bits::new(s_196_3 as u128, 2u16);
        // D s_196_5: cmp-ne s_196_1 s_196_4
        let s_196_5: bool = ((s_196_1) != (s_196_4));
        // D s_196_6: write-var gs#18787 <= s_196_5
        fn_state.gs_18787 = s_196_5;
        // N s_196_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_197_0: read-var accdesc.1:struct
        let s_197_0: u32 = fn_state.accdesc._1;
        // C s_197_1: const #11u : u32
        let s_197_1: u32 = 11;
        // D s_197_2: cmp-eq s_197_0 s_197_1
        let s_197_2: bool = ((s_197_0) == (s_197_1));
        // D s_197_3: write-var gs#18786 <= s_197_2
        fn_state.gs_18786 = s_197_2;
        // N s_197_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_198_0: const #15328u : u32
        let s_198_0: u32 = 15328;
        // D s_198_1: read-reg s_198_0:struct
        let s_198_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_198_0 as isize);
            tracer.read_register(s_198_0 as isize, value);
            value
        };
        // D s_198_2: call _get_VTCR_EL2_Type_GCSH(s_198_1)
        let s_198_2: bool = u_get_VTCR_EL2_Type_GCSH(state, tracer, s_198_1);
        // D s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 1u16);
        // C s_198_4: const #1u : u8
        let s_198_4: bool = true;
        // C s_198_5: cast zx s_198_4 -> bv
        let s_198_5: Bits = Bits::new(s_198_4 as u128, 1u16);
        // D s_198_6: cmp-eq s_198_3 s_198_5
        let s_198_6: bool = ((s_198_3) == (s_198_5));
        // D s_198_7: write-var gs#18785 <= s_198_6
        fn_state.gs_18785 = s_198_6;
        // N s_198_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_199_0: const #() : ()
        let s_199_0: () = ();
        // S s_199_1: call HaveGCS(s_199_0)
        let s_199_1: bool = HaveGCS(state, tracer, s_199_0);
        // D s_199_2: write-var gs#18784 <= s_199_1
        fn_state.gs_18784 = s_199_1;
        // N s_199_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_200_0: const #1u : u8
        let s_200_0: bool = true;
        // D s_200_1: write-var gs#18788 <= s_200_0
        fn_state.gs_18788 = s_200_0;
        // N s_200_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_201_0: read-var ipa.4:struct
        let s_201_0: bool = fn_state.ipa._4;
        // D s_201_1: not s_201_0
        let s_201_1: bool = !s_201_0;
        // D s_201_2: write-var gs#18783 <= s_201_1
        fn_state.gs_18783 = s_201_1;
        // N s_201_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_202_0: read-var accdesc.29:struct
        let s_202_0: bool = fn_state.accdesc._29;
        // N s_202_1: branch s_202_0 b277 b203
        if s_202_0 {
            return block_277(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_203_0: const #0u : u8
        let s_203_0: bool = false;
        // D s_203_1: write-var gs#18849 <= s_203_0
        fn_state.gs_18849 = s_203_0;
        // N s_203_2: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_204_0: read-var gs#18849:u8
        let s_204_0: bool = fn_state.gs_18849;
        // N s_204_1: branch s_204_0 b267 b205
        if s_204_0 {
            return block_267(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_205_0: const #0u : u8
        let s_205_0: bool = false;
        // D s_205_1: write-var gs#18853 <= s_205_0
        fn_state.gs_18853 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_206_0: read-var gs#18853:u8
        let s_206_0: bool = fn_state.gs_18853;
        // N s_206_1: branch s_206_0 b266 b207
        if s_206_0 {
            return block_266(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_207_0: read-var accdesc.29:struct
        let s_207_0: bool = fn_state.accdesc._29;
        // N s_207_1: branch s_207_0 b265 b208
        if s_207_0 {
            return block_265(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_208_0: const #0u : u8
        let s_208_0: bool = false;
        // D s_208_1: write-var gs#18854 <= s_208_0
        fn_state.gs_18854 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_209_0: read-var gs#18854:u8
        let s_209_0: bool = fn_state.gs_18854;
        // N s_209_1: branch s_209_0 b255 b210
        if s_209_0 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_210(state, tracer, fn_state);
        };
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_210_0: const #0u : u8
        let s_210_0: bool = false;
        // D s_210_1: write-var gs#18858 <= s_210_0
        fn_state.gs_18858 = s_210_0;
        // N s_210_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_211_0: read-var gs#18858:u8
        let s_211_0: bool = fn_state.gs_18858;
        // N s_211_1: branch s_211_0 b254 b212
        if s_211_0 {
            return block_254(state, tracer, fn_state);
        } else {
            return block_212(state, tracer, fn_state);
        };
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_212_0: read-var walkparams.15:struct
        let s_212_0: bool = fn_state.walkparams._15;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 1u16);
        // C s_212_2: const #1u : u8
        let s_212_2: bool = true;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // N s_212_5: branch s_212_4 b253 b213
        if s_212_4 {
            return block_253(state, tracer, fn_state);
        } else {
            return block_213(state, tracer, fn_state);
        };
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_213_0: const #0u : u8
        let s_213_0: bool = false;
        // D s_213_1: write-var gs#18859 <= s_213_0
        fn_state.gs_18859 = s_213_0;
        // N s_213_2: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_214_0: read-var gs#18859:u8
        let s_214_0: bool = fn_state.gs_18859;
        // N s_214_1: branch s_214_0 b252 b215
        if s_214_0 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_215_0: read-var s2perms.3:struct
        let s_215_0: bool = fn_state.s2perms._3;
        // N s_215_1: branch s_215_0 b251 b216
        if s_215_0 {
            return block_251(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_216_0: const #0u : u8
        let s_216_0: bool = false;
        // D s_216_1: write-var gs#18860 <= s_216_0
        fn_state.gs_18860 = s_216_0;
        // N s_216_2: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_217_0: read-var gs#18860:u8
        let s_217_0: bool = fn_state.gs_18860;
        // N s_217_1: branch s_217_0 b250 b218
        if s_217_0 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_218_0: read-var accdesc.32:struct
        let s_218_0: bool = fn_state.accdesc._32;
        // N s_218_1: branch s_218_0 b249 b219
        if s_218_0 {
            return block_249(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#18861 <= s_219_0
        fn_state.gs_18861 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_220_0: read-var gs#18861:u8
        let s_220_0: bool = fn_state.gs_18861;
        // N s_220_1: branch s_220_0 b248 b221
        if s_220_0 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_221_0: const #0u : u8
        let s_221_0: bool = false;
        // D s_221_1: write-var gs#18862 <= s_221_0
        fn_state.gs_18862 = s_221_0;
        // N s_221_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_222_0: read-var gs#18862:u8
        let s_222_0: bool = fn_state.gs_18862;
        // N s_222_1: branch s_222_0 b247 b223
        if s_222_0 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_223(state, tracer, fn_state);
        };
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_223_0: read-var accdesc.25:struct
        let s_223_0: u32 = fn_state.accdesc._25;
        // C s_223_1: const #2u : u32
        let s_223_1: u32 = 2;
        // D s_223_2: cmp-eq s_223_0 s_223_1
        let s_223_2: bool = ((s_223_0) == (s_223_1));
        // N s_223_3: branch s_223_2 b246 b224
        if s_223_2 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_224(state, tracer, fn_state);
        };
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_224_0: const #0u : u8
        let s_224_0: bool = false;
        // D s_224_1: write-var gs#18863 <= s_224_0
        fn_state.gs_18863 = s_224_0;
        // N s_224_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_225_0: read-var gs#18863:u8
        let s_225_0: bool = fn_state.gs_18863;
        // N s_225_1: branch s_225_0 b245 b226
        if s_225_0 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_226_0: read-var r:u8
        let s_226_0: bool = fn_state.r;
        // D s_226_1: cast zx s_226_0 -> bv
        let s_226_1: Bits = Bits::new(s_226_0 as u128, 1u16);
        // C s_226_2: const #0u : u8
        let s_226_2: bool = false;
        // C s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // D s_226_4: cmp-eq s_226_1 s_226_3
        let s_226_4: bool = ((s_226_1) == (s_226_3));
        // N s_226_5: branch s_226_4 b244 b227
        if s_226_4 {
            return block_244(state, tracer, fn_state);
        } else {
            return block_227(state, tracer, fn_state);
        };
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_227_0: read-var accdesc.32:struct
        let s_227_0: bool = fn_state.accdesc._32;
        // N s_227_1: branch s_227_0 b243 b228
        if s_227_0 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_228_0: const #0u : u8
        let s_228_0: bool = false;
        // D s_228_1: write-var gs#18864 <= s_228_0
        fn_state.gs_18864 = s_228_0;
        // N s_228_2: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_229_0: read-var gs#18864:u8
        let s_229_0: bool = fn_state.gs_18864;
        // N s_229_1: branch s_229_0 b242 b230
        if s_229_0 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_230(state, tracer, fn_state);
        };
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_230_0: read-var accdesc.32:struct
        let s_230_0: bool = fn_state.accdesc._32;
        // N s_230_1: branch s_230_0 b241 b231
        if s_230_0 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_231_0: const #0u : u8
        let s_231_0: bool = false;
        // D s_231_1: write-var gs#18865 <= s_231_0
        fn_state.gs_18865 = s_231_0;
        // N s_231_2: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_232_0: read-var gs#18865:u8
        let s_232_0: bool = fn_state.gs_18865;
        // N s_232_1: branch s_232_0 b240 b233
        if s_232_0 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_233(state, tracer, fn_state);
        };
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_233_0: const #0u : u8
        let s_233_0: bool = false;
        // D s_233_1: write-var gs#18866 <= s_233_0
        fn_state.gs_18866 = s_233_0;
        // N s_233_2: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_234_0: read-var gs#18866:u8
        let s_234_0: bool = fn_state.gs_18866;
        // N s_234_1: branch s_234_0 b239 b235
        if s_234_0 {
            return block_239(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_235_0: const #0u : u8
        let s_235_0: bool = false;
        // D s_235_1: write-var gs#18867 <= s_235_0
        fn_state.gs_18867 = s_235_0;
        // N s_235_2: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_236_0: read-var gs#18867:u8
        let s_236_0: bool = fn_state.gs_18867;
        // N s_236_1: branch s_236_0 b238 b237
        if s_236_0 {
            return block_238(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // N s_237_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_238_0: const #5u : u32
        let s_238_0: u32 = 5;
        // D s_238_1: write-var fault.16 <= s_238_0
        fn_state.fault._16 = s_238_0;
        // C s_238_2: const #1u : u8
        let s_238_2: bool = true;
        // D s_238_3: write-var fault.3 <= s_238_2
        fn_state.fault._3 = s_238_2;
        // N s_238_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_239_0: read-var permissions.8:struct
        let s_239_0: bool = fn_state.permissions._8;
        // D s_239_1: cast zx s_239_0 -> bv
        let s_239_1: Bits = Bits::new(s_239_0 as u128, 1u16);
        // C s_239_2: const #0u : u8
        let s_239_2: bool = false;
        // C s_239_3: cast zx s_239_2 -> bv
        let s_239_3: Bits = Bits::new(s_239_2 as u128, 1u16);
        // D s_239_4: cmp-eq s_239_1 s_239_3
        let s_239_4: bool = ((s_239_1) == (s_239_3));
        // D s_239_5: write-var gs#18867 <= s_239_4
        fn_state.gs_18867 = s_239_4;
        // N s_239_6: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_240_0: read-var walkparams.17:struct
        let s_240_0: bool = fn_state.walkparams._17;
        // D s_240_1: cast zx s_240_0 -> bv
        let s_240_1: Bits = Bits::new(s_240_0 as u128, 1u16);
        // C s_240_2: const #1u : u8
        let s_240_2: bool = true;
        // C s_240_3: cast zx s_240_2 -> bv
        let s_240_3: Bits = Bits::new(s_240_2 as u128, 1u16);
        // D s_240_4: cmp-eq s_240_1 s_240_3
        let s_240_4: bool = ((s_240_1) == (s_240_3));
        // D s_240_5: write-var gs#18866 <= s_240_4
        fn_state.gs_18866 = s_240_4;
        // N s_240_6: jump b234
        return block_234(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_241_0: read-var walkparams.7:struct
        let s_241_0: bool = fn_state.walkparams._7;
        // D s_241_1: read-var walkparams.9:struct
        let s_241_1: bool = fn_state.walkparams._9;
        // D s_241_2: cast zx s_241_0 -> bv
        let s_241_2: Bits = Bits::new(s_241_0 as u128, 1u16);
        // D s_241_3: cast zx s_241_1 -> bv
        let s_241_3: Bits = Bits::new(s_241_1 as u128, 1u16);
        // D s_241_4: cast reint s_241_2 -> u128
        let s_241_4: u128 = (s_241_2.value() as u128);
        // D s_241_5: size-of s_241_2
        let s_241_5: u16 = s_241_2.length();
        // D s_241_6: cast reint s_241_3 -> u128
        let s_241_6: u128 = (s_241_3.value() as u128);
        // D s_241_7: size-of s_241_3
        let s_241_7: u16 = s_241_3.length();
        // D s_241_8: lsl s_241_4 s_241_7
        let s_241_8: u128 = s_241_4 << s_241_7;
        // D s_241_9: or s_241_8 s_241_6
        let s_241_9: u128 = ((s_241_8) | (s_241_6));
        // D s_241_10: add s_241_5 s_241_7
        let s_241_10: u16 = (s_241_5 + s_241_7);
        // D s_241_11: create-bits s_241_9 s_241_10
        let s_241_11: Bits = Bits::new(s_241_9, s_241_10);
        // D s_241_12: cast reint s_241_11 -> u8
        let s_241_12: u8 = (s_241_11.value() as u8);
        // D s_241_13: cast zx s_241_12 -> bv
        let s_241_13: Bits = Bits::new(s_241_12 as u128, 2u16);
        // C s_241_14: const #3u : u8
        let s_241_14: u8 = 3;
        // C s_241_15: cast zx s_241_14 -> bv
        let s_241_15: Bits = Bits::new(s_241_14 as u128, 2u16);
        // D s_241_16: cmp-eq s_241_13 s_241_15
        let s_241_16: bool = ((s_241_13) == (s_241_15));
        // D s_241_17: not s_241_16
        let s_241_17: bool = !s_241_16;
        // D s_241_18: write-var gs#18865 <= s_241_17
        fn_state.gs_18865 = s_241_17;
        // N s_241_19: jump b232
        return block_232(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_242_0: const #5u : u32
        let s_242_0: u32 = 5;
        // D s_242_1: write-var fault.16 <= s_242_0
        fn_state.fault._16 = s_242_0;
        // N s_242_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_243_0: read-var w:u8
        let s_243_0: bool = fn_state.w;
        // D s_243_1: cast zx s_243_0 -> bv
        let s_243_1: Bits = Bits::new(s_243_0 as u128, 1u16);
        // C s_243_2: const #0u : u8
        let s_243_2: bool = false;
        // C s_243_3: cast zx s_243_2 -> bv
        let s_243_3: Bits = Bits::new(s_243_2 as u128, 1u16);
        // D s_243_4: cmp-eq s_243_1 s_243_3
        let s_243_4: bool = ((s_243_1) == (s_243_3));
        // D s_243_5: write-var gs#18864 <= s_243_4
        fn_state.gs_18864 = s_243_4;
        // N s_243_6: jump b229
        return block_229(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_244_0: const #5u : u32
        let s_244_0: u32 = 5;
        // D s_244_1: write-var fault.16 <= s_244_0
        fn_state.fault._16 = s_244_0;
        // N s_244_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_245_0: const #5u : u32
        let s_245_0: u32 = 5;
        // D s_245_1: write-var fault.16 <= s_245_0
        fn_state.fault._16 = s_245_0;
        // N s_245_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_246_0: read-var walkstate.0:struct
        let s_246_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_246_1: write-var ga#13962 <= s_246_0
        fn_state.ga_13962 = s_246_0;
        // D s_246_2: read-var ga#13962.1:struct
        let s_246_2: u32 = fn_state.ga_13962._1;
        // C s_246_3: const #3u : u32
        let s_246_3: u32 = 3;
        // D s_246_4: cmp-eq s_246_2 s_246_3
        let s_246_4: bool = ((s_246_2) == (s_246_3));
        // D s_246_5: write-var gs#18863 <= s_246_4
        fn_state.gs_18863 = s_246_4;
        // N s_246_6: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_247_0: const #5u : u32
        let s_247_0: u32 = 5;
        // D s_247_1: write-var fault.16 <= s_247_0
        fn_state.fault._16 = s_247_0;
        // C s_247_2: const #1u : u8
        let s_247_2: bool = true;
        // D s_247_3: write-var fault.11 <= s_247_2
        fn_state.fault._11 = s_247_2;
        // N s_247_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_248_0: read-var ow:u8
        let s_248_0: bool = fn_state.ow;
        // D s_248_1: cast zx s_248_0 -> bv
        let s_248_1: Bits = Bits::new(s_248_0 as u128, 1u16);
        // C s_248_2: const #0u : u8
        let s_248_2: bool = false;
        // C s_248_3: cast zx s_248_2 -> bv
        let s_248_3: Bits = Bits::new(s_248_2 as u128, 1u16);
        // D s_248_4: cmp-eq s_248_1 s_248_3
        let s_248_4: bool = ((s_248_1) == (s_248_3));
        // D s_248_5: write-var gs#18862 <= s_248_4
        fn_state.gs_18862 = s_248_4;
        // N s_248_6: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_249_0: read-var s2perms.3:struct
        let s_249_0: bool = fn_state.s2perms._3;
        // D s_249_1: write-var gs#18861 <= s_249_0
        fn_state.gs_18861 = s_249_0;
        // N s_249_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_250_0: const #5u : u32
        let s_250_0: u32 = 5;
        // D s_250_1: write-var fault.16 <= s_250_0
        fn_state.fault._16 = s_250_0;
        // C s_250_2: const #1u : u8
        let s_250_2: bool = true;
        // D s_250_3: write-var fault.11 <= s_250_2
        fn_state.fault._11 = s_250_2;
        // N s_250_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_251_0: read-var or:u8
        let s_251_0: bool = fn_state.or;
        // D s_251_1: cast zx s_251_0 -> bv
        let s_251_1: Bits = Bits::new(s_251_0 as u128, 1u16);
        // C s_251_2: const #0u : u8
        let s_251_2: bool = false;
        // C s_251_3: cast zx s_251_2 -> bv
        let s_251_3: Bits = Bits::new(s_251_2 as u128, 1u16);
        // D s_251_4: cmp-eq s_251_1 s_251_3
        let s_251_4: bool = ((s_251_1) == (s_251_3));
        // D s_251_5: write-var gs#18860 <= s_251_4
        fn_state.gs_18860 = s_251_4;
        // N s_251_6: jump b217
        return block_217(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_252_0: const #5u : u32
        let s_252_0: u32 = 5;
        // D s_252_1: write-var fault.16 <= s_252_0
        fn_state.fault._16 = s_252_0;
        // N s_252_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_253_0: read-var memtype:u32
        let s_253_0: u32 = fn_state.memtype;
        // C s_253_1: const #1u : u32
        let s_253_1: u32 = 1;
        // D s_253_2: cmp-eq s_253_0 s_253_1
        let s_253_2: bool = ((s_253_0) == (s_253_1));
        // D s_253_3: write-var gs#18859 <= s_253_2
        fn_state.gs_18859 = s_253_2;
        // N s_253_4: jump b214
        return block_214(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_254_0: const #5u : u32
        let s_254_0: u32 = 5;
        // D s_254_1: write-var fault.16 <= s_254_0
        fn_state.fault._16 = s_254_0;
        // C s_254_2: const #1u : u8
        let s_254_2: bool = true;
        // D s_254_3: write-var fault.18 <= s_254_2
        fn_state.fault._18 = s_254_2;
        // N s_254_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_255_0: read-var walkparams.28:struct
        let s_255_0: bool = fn_state.walkparams._28;
        // D s_255_1: cast zx s_255_0 -> bv
        let s_255_1: Bits = Bits::new(s_255_0 as u128, 1u16);
        // C s_255_2: const #1u : u8
        let s_255_2: bool = true;
        // C s_255_3: cast zx s_255_2 -> bv
        let s_255_3: Bits = Bits::new(s_255_2 as u128, 1u16);
        // D s_255_4: cmp-eq s_255_1 s_255_3
        let s_255_4: bool = ((s_255_1) == (s_255_3));
        // N s_255_5: branch s_255_4 b264 b256
        if s_255_4 {
            return block_264(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_256_0: const #0u : u8
        let s_256_0: bool = false;
        // D s_256_1: write-var gs#18855 <= s_256_0
        fn_state.gs_18855 = s_256_0;
        // N s_256_2: jump b257
        return block_257(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_257_0: read-var gs#18855:u8
        let s_257_0: bool = fn_state.gs_18855;
        // N s_257_1: branch s_257_0 b263 b258
        if s_257_0 {
            return block_263(state, tracer, fn_state);
        } else {
            return block_258(state, tracer, fn_state);
        };
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_258_0: read-var walkparams.27:struct
        let s_258_0: bool = fn_state.walkparams._27;
        // D s_258_1: cast zx s_258_0 -> bv
        let s_258_1: Bits = Bits::new(s_258_0 as u128, 1u16);
        // C s_258_2: const #1u : u8
        let s_258_2: bool = true;
        // C s_258_3: cast zx s_258_2 -> bv
        let s_258_3: Bits = Bits::new(s_258_2 as u128, 1u16);
        // D s_258_4: cmp-eq s_258_1 s_258_3
        let s_258_4: bool = ((s_258_1) == (s_258_3));
        // N s_258_5: branch s_258_4 b262 b259
        if s_258_4 {
            return block_262(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_259_0: const #0u : u8
        let s_259_0: bool = false;
        // D s_259_1: write-var gs#18856 <= s_259_0
        fn_state.gs_18856 = s_259_0;
        // N s_259_2: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_260_0: read-var gs#18856:u8
        let s_260_0: bool = fn_state.gs_18856;
        // D s_260_1: write-var gs#18857 <= s_260_0
        fn_state.gs_18857 = s_260_0;
        // N s_260_2: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_261_0: read-var gs#18857:u8
        let s_261_0: bool = fn_state.gs_18857;
        // D s_261_1: write-var gs#18858 <= s_261_0
        fn_state.gs_18858 = s_261_0;
        // N s_261_2: jump b211
        return block_211(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_262_0: read-var s2perms.12:struct
        let s_262_0: bool = fn_state.s2perms._12;
        // D s_262_1: read-var s2perms.11:struct
        let s_262_1: bool = fn_state.s2perms._11;
        // D s_262_2: cast zx s_262_0 -> bv
        let s_262_2: Bits = Bits::new(s_262_0 as u128, 1u16);
        // D s_262_3: cast zx s_262_1 -> bv
        let s_262_3: Bits = Bits::new(s_262_1 as u128, 1u16);
        // D s_262_4: cast reint s_262_2 -> u128
        let s_262_4: u128 = (s_262_2.value() as u128);
        // D s_262_5: size-of s_262_2
        let s_262_5: u16 = s_262_2.length();
        // D s_262_6: cast reint s_262_3 -> u128
        let s_262_6: u128 = (s_262_3.value() as u128);
        // D s_262_7: size-of s_262_3
        let s_262_7: u16 = s_262_3.length();
        // D s_262_8: lsl s_262_4 s_262_7
        let s_262_8: u128 = s_262_4 << s_262_7;
        // D s_262_9: or s_262_8 s_262_6
        let s_262_9: u128 = ((s_262_8) | (s_262_6));
        // D s_262_10: add s_262_5 s_262_7
        let s_262_10: u16 = (s_262_5 + s_262_7);
        // D s_262_11: create-bits s_262_9 s_262_10
        let s_262_11: Bits = Bits::new(s_262_9, s_262_10);
        // D s_262_12: cast reint s_262_11 -> u8
        let s_262_12: u8 = (s_262_11.value() as u8);
        // D s_262_13: cast zx s_262_12 -> bv
        let s_262_13: Bits = Bits::new(s_262_12 as u128, 2u16);
        // C s_262_14: const #1u : u8
        let s_262_14: u8 = 1;
        // C s_262_15: cast zx s_262_14 -> bv
        let s_262_15: Bits = Bits::new(s_262_14 as u128, 2u16);
        // D s_262_16: cmp-eq s_262_13 s_262_15
        let s_262_16: bool = ((s_262_13) == (s_262_15));
        // D s_262_17: write-var gs#18856 <= s_262_16
        fn_state.gs_18856 = s_262_16;
        // N s_262_18: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_263_0: const #1u : u8
        let s_263_0: bool = true;
        // D s_263_1: write-var gs#18857 <= s_263_0
        fn_state.gs_18857 = s_263_0;
        // N s_263_2: jump b261
        return block_261(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_264_0: read-var s2perms.12:struct
        let s_264_0: bool = fn_state.s2perms._12;
        // D s_264_1: cast zx s_264_0 -> bv
        let s_264_1: Bits = Bits::new(s_264_0 as u128, 1u16);
        // C s_264_2: const #0u : u8
        let s_264_2: bool = false;
        // C s_264_3: cast zx s_264_2 -> bv
        let s_264_3: Bits = Bits::new(s_264_2 as u128, 1u16);
        // D s_264_4: cmp-eq s_264_1 s_264_3
        let s_264_4: bool = ((s_264_1) == (s_264_3));
        // D s_264_5: write-var gs#18855 <= s_264_4
        fn_state.gs_18855 = s_264_4;
        // N s_264_6: jump b257
        return block_257(state, tracer, fn_state);
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_265_0: read-var accdesc.31:struct
        let s_265_0: u32 = fn_state.accdesc._31;
        // C s_265_1: const #1u : u32
        let s_265_1: u32 = 1;
        // D s_265_2: cmp-eq s_265_0 s_265_1
        let s_265_2: bool = ((s_265_0) == (s_265_1));
        // D s_265_3: write-var gs#18854 <= s_265_2
        fn_state.gs_18854 = s_265_2;
        // N s_265_4: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_266_0: const #5u : u32
        let s_266_0: u32 = 5;
        // D s_266_1: write-var fault.16 <= s_266_0
        fn_state.fault._16 = s_266_0;
        // C s_266_2: const #1u : u8
        let s_266_2: bool = true;
        // D s_266_3: write-var fault.18 <= s_266_2
        fn_state.fault._18 = s_266_2;
        // N s_266_4: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_267_0: read-var walkparams.27:struct
        let s_267_0: bool = fn_state.walkparams._27;
        // D s_267_1: cast zx s_267_0 -> bv
        let s_267_1: Bits = Bits::new(s_267_0 as u128, 1u16);
        // C s_267_2: const #1u : u8
        let s_267_2: bool = true;
        // C s_267_3: cast zx s_267_2 -> bv
        let s_267_3: Bits = Bits::new(s_267_2 as u128, 1u16);
        // D s_267_4: cmp-eq s_267_1 s_267_3
        let s_267_4: bool = ((s_267_1) == (s_267_3));
        // N s_267_5: branch s_267_4 b276 b268
        if s_267_4 {
            return block_276(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_268_0: const #0u : u8
        let s_268_0: bool = false;
        // D s_268_1: write-var gs#18850 <= s_268_0
        fn_state.gs_18850 = s_268_0;
        // N s_268_2: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_269_0: read-var gs#18850:u8
        let s_269_0: bool = fn_state.gs_18850;
        // N s_269_1: branch s_269_0 b275 b270
        if s_269_0 {
            return block_275(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_270_0: read-var walkparams.28:struct
        let s_270_0: bool = fn_state.walkparams._28;
        // D s_270_1: cast zx s_270_0 -> bv
        let s_270_1: Bits = Bits::new(s_270_0 as u128, 1u16);
        // C s_270_2: const #1u : u8
        let s_270_2: bool = true;
        // C s_270_3: cast zx s_270_2 -> bv
        let s_270_3: Bits = Bits::new(s_270_2 as u128, 1u16);
        // D s_270_4: cmp-eq s_270_1 s_270_3
        let s_270_4: bool = ((s_270_1) == (s_270_3));
        // N s_270_5: branch s_270_4 b274 b271
        if s_270_4 {
            return block_274(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_271_0: const #0u : u8
        let s_271_0: bool = false;
        // D s_271_1: write-var gs#18851 <= s_271_0
        fn_state.gs_18851 = s_271_0;
        // N s_271_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_272_0: read-var gs#18851:u8
        let s_272_0: bool = fn_state.gs_18851;
        // D s_272_1: write-var gs#18852 <= s_272_0
        fn_state.gs_18852 = s_272_0;
        // N s_272_2: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_273_0: read-var gs#18852:u8
        let s_273_0: bool = fn_state.gs_18852;
        // D s_273_1: write-var gs#18853 <= s_273_0
        fn_state.gs_18853 = s_273_0;
        // N s_273_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_274_0: read-var s2perms.12:struct
        let s_274_0: bool = fn_state.s2perms._12;
        // D s_274_1: read-var s2perms.11:struct
        let s_274_1: bool = fn_state.s2perms._11;
        // D s_274_2: cast zx s_274_0 -> bv
        let s_274_2: Bits = Bits::new(s_274_0 as u128, 1u16);
        // D s_274_3: cast zx s_274_1 -> bv
        let s_274_3: Bits = Bits::new(s_274_1 as u128, 1u16);
        // D s_274_4: cast reint s_274_2 -> u128
        let s_274_4: u128 = (s_274_2.value() as u128);
        // D s_274_5: size-of s_274_2
        let s_274_5: u16 = s_274_2.length();
        // D s_274_6: cast reint s_274_3 -> u128
        let s_274_6: u128 = (s_274_3.value() as u128);
        // D s_274_7: size-of s_274_3
        let s_274_7: u16 = s_274_3.length();
        // D s_274_8: lsl s_274_4 s_274_7
        let s_274_8: u128 = s_274_4 << s_274_7;
        // D s_274_9: or s_274_8 s_274_6
        let s_274_9: u128 = ((s_274_8) | (s_274_6));
        // D s_274_10: add s_274_5 s_274_7
        let s_274_10: u16 = (s_274_5 + s_274_7);
        // D s_274_11: create-bits s_274_9 s_274_10
        let s_274_11: Bits = Bits::new(s_274_9, s_274_10);
        // D s_274_12: cast reint s_274_11 -> u8
        let s_274_12: u8 = (s_274_11.value() as u8);
        // D s_274_13: cast zx s_274_12 -> bv
        let s_274_13: Bits = Bits::new(s_274_12 as u128, 2u16);
        // C s_274_14: const #2u : u8
        let s_274_14: u8 = 2;
        // C s_274_15: cast zx s_274_14 -> bv
        let s_274_15: Bits = Bits::new(s_274_14 as u128, 2u16);
        // D s_274_16: cmp-eq s_274_13 s_274_15
        let s_274_16: bool = ((s_274_13) == (s_274_15));
        // D s_274_17: write-var gs#18851 <= s_274_16
        fn_state.gs_18851 = s_274_16;
        // N s_274_18: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // C s_275_0: const #1u : u8
        let s_275_0: bool = true;
        // D s_275_1: write-var gs#18852 <= s_275_0
        fn_state.gs_18852 = s_275_0;
        // N s_275_2: jump b273
        return block_273(state, tracer, fn_state);
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_276_0: read-var s2perms.11:struct
        let s_276_0: bool = fn_state.s2perms._11;
        // D s_276_1: cast zx s_276_0 -> bv
        let s_276_1: Bits = Bits::new(s_276_0 as u128, 1u16);
        // C s_276_2: const #0u : u8
        let s_276_2: bool = false;
        // C s_276_3: cast zx s_276_2 -> bv
        let s_276_3: Bits = Bits::new(s_276_2 as u128, 1u16);
        // D s_276_4: cmp-eq s_276_1 s_276_3
        let s_276_4: bool = ((s_276_1) == (s_276_3));
        // D s_276_5: write-var gs#18850 <= s_276_4
        fn_state.gs_18850 = s_276_4;
        // N s_276_6: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_277_0: read-var accdesc.31:struct
        let s_277_0: u32 = fn_state.accdesc._31;
        // C s_277_1: const #0u : u32
        let s_277_1: u32 = 0;
        // D s_277_2: cmp-eq s_277_0 s_277_1
        let s_277_2: bool = ((s_277_0) == (s_277_1));
        // D s_277_3: write-var gs#18849 <= s_277_2
        fn_state.gs_18849 = s_277_2;
        // N s_277_4: jump b204
        return block_204(state, tracer, fn_state);
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_278_0: read-var s2perms.10:struct
        let s_278_0: bool = fn_state.s2perms._10;
        // D s_278_1: write-var r <= s_278_0
        fn_state.r = s_278_0;
        // D s_278_2: read-var s2perms.15:struct
        let s_278_2: bool = fn_state.s2perms._15;
        // D s_278_3: write-var w <= s_278_2
        fn_state.w = s_278_2;
        // D s_278_4: read-var s2perms.2:struct
        let s_278_4: bool = fn_state.s2perms._2;
        // D s_278_5: write-var or <= s_278_4
        fn_state.or = s_278_4;
        // D s_278_6: read-var s2perms.6:struct
        let s_278_6: bool = fn_state.s2perms._6;
        // D s_278_7: write-var ow <= s_278_6
        fn_state.ow = s_278_6;
        // N s_278_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType3b8bd97143a1dd5c {
        // D s_279_0: read-var s2perms.9:struct
        let s_279_0: bool = fn_state.s2perms._9;
        // D s_279_1: write-var r <= s_279_0
        fn_state.r = s_279_0;
        // D s_279_2: read-var s2perms.14:struct
        let s_279_2: bool = fn_state.s2perms._14;
        // D s_279_3: write-var w <= s_279_2
        fn_state.w = s_279_2;
        // D s_279_4: read-var s2perms.1:struct
        let s_279_4: bool = fn_state.s2perms._1;
        // D s_279_5: write-var or <= s_279_4
        fn_state.or = s_279_4;
        // D s_279_6: read-var s2perms.5:struct
        let s_279_6: bool = fn_state.s2perms._5;
        // D s_279_7: write-var ow <= s_279_6
        fn_state.ow = s_279_6;
        // N s_279_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
