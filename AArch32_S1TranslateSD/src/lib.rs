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
use AArch32_S1SDHasPermissionsFault::*;
use AArch32_S1Enabled::*;
use SetInGuardedPage::*;
use AArch32_EL2Enabled::*;
use HCR_read::*;
use AArch32_S1DCacheEnabled::*;
use HaveAArch32EL::*;
use SCTLR_NS_read::*;
use u__UNKNOWN_SDFType::*;
use u__IMPDEF_boolean::*;
use EffectiveShareability::*;
use AArch32_S1HasAlignmentFault::*;
use CreateAddressDescriptor::*;
use u_get_SCTLR_Type_nTLSMD::*;
use u_get_HCR_Type_VM::*;
use ELStateUsingAArch32::*;
use SCTLR_read__2::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use AArch32_S1WalkSD::*;
use AArch32_S1ICacheEnabled::*;
use AArch32_SDStageOA::*;
use NormalNCMemAttr::*;
use u__UNKNOWN_AddressDescriptor::*;
use AArch32_S1DisabledOutput::*;
use u_get_HCR_EL2_Type_VM::*;
use AArch32_OutputDomain::*;
use common::*;
pub fn AArch32_S1TranslateSD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    va: u32,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductType234df14d4fab6c9d {
    #[derive(Default)]
    struct FunctionState {
        gs_29022: bool,
        domain: u8,
        fault: ProductType1d757adad216cdef,
        ga_22364: ProductType201519a0f62623dc,
        ntlsmd: bool,
        gs_29033: bool,
        ga_22361: ProductTypedc31059ca7e2391c,
        gs_29036: bool,
        gs_29030: bool,
        gs_29031: bool,
        walkstate: ProductType96e7acababe246a1,
        ga_22415: bool,
        gs_29023: bool,
        ga_22408: ProductTypef170cab34335b70c,
        ga_22401: ProductTypef170cab34335b70c,
        memattrs: ProductTypef170cab34335b70c,
        ga_22384: ProductTypeda0231e9dc169f81,
        gs_29035: bool,
        return_value: ProductType234df14d4fab6c9d,
        gs_29032: bool,
        gs_29034: bool,
        ga_22395: ProductTypef170cab34335b70c,
        ga_22420: ProductTypef170cab34335b70c,
        ga_22383: ProductTypef170cab34335b70c,
        gs_29037: bool,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        va: u32,
        aligned: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        va,
        aligned,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // D s_0_2: read-var accdesc.25:struct
        let s_0_2: u32 = fn_state.accdesc._25;
        // D s_0_3: read-var regime:u32
        let s_0_3: u32 = fn_state.regime;
        // D s_0_4: call AArch32_S1Enabled(s_0_3, s_0_2)
        let s_0_4: bool = AArch32_S1Enabled(state, tracer, s_0_3, s_0_2);
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b67 b1
        if s_0_5 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_1_0: read-var fault:struct
        let s_1_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_1_1: read-var regime:u32
        let s_1_1: u32 = fn_state.regime;
        // D s_1_2: read-var accdesc:struct
        let s_1_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_3: read-var va:u32
        let s_1_3: u32 = fn_state.va;
        // D s_1_4: call AArch32_S1WalkSD(s_1_0, s_1_1, s_1_2, s_1_3)
        let s_1_4: ProductType201519a0f62623dc = AArch32_S1WalkSD(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
            s_1_3,
        );
        // D s_1_5: write-var ga#22364 <= s_1_4
        fn_state.ga_22364 = s_1_4;
        // D s_1_6: read-var ga#22364.0:struct
        let s_1_6: ProductType1d757adad216cdef = fn_state.ga_22364._0;
        // D s_1_7: read-var ga#22364.1:struct
        let s_1_7: ProductType96e7acababe246a1 = fn_state.ga_22364._1;
        // D s_1_8: write-var fault <= s_1_6
        fn_state.fault = s_1_6;
        // D s_1_9: write-var walkstate <= s_1_7
        fn_state.walkstate = s_1_7;
        // D s_1_10: read-var fault.16:struct
        let s_1_10: u32 = fn_state.fault._16;
        // C s_1_11: const #0u : u32
        let s_1_11: u32 = 0;
        // D s_1_12: cmp-eq s_1_10 s_1_11
        let s_1_12: bool = ((s_1_10) == (s_1_11));
        // N s_1_13: branch s_1_12 b66 b2
        if s_1_12 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_2_0: read-var walkstate.3:struct
        let s_2_0: u8 = fn_state.walkstate._3;
        // D s_2_1: read-var regime:u32
        let s_2_1: u32 = fn_state.regime;
        // D s_2_2: call AArch32_OutputDomain(s_2_1, s_2_0)
        let s_2_2: u8 = AArch32_OutputDomain(state, tracer, s_2_1, s_2_0);
        // D s_2_3: write-var domain <= s_2_2
        fn_state.domain = s_2_2;
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // S s_2_5: call SetInGuardedPage(s_2_4)
        let s_2_5: () = SetInGuardedPage(state, tracer, s_2_4);
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call HaveTrapLoadStoreMultipleDeviceExt(s_2_6)
        let s_2_7: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_2_6);
        // N s_2_8: branch s_2_7 b57 b3
        if s_2_7 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var ntlsmd <= s_3_0
        fn_state.ntlsmd = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_4_0: read-var walkstate.7:struct
        let s_4_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_4_1: read-var accdesc:struct
        let s_4_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_2: read-var aligned:u8
        let s_4_2: bool = fn_state.aligned;
        // D s_4_3: read-var ntlsmd:u8
        let s_4_3: bool = fn_state.ntlsmd;
        // D s_4_4: call AArch32_S1HasAlignmentFault(s_4_1, s_4_2, s_4_3, s_4_0)
        let s_4_4: bool = AArch32_S1HasAlignmentFault(
            state,
            tracer,
            s_4_1,
            s_4_2,
            s_4_3,
            s_4_0,
        );
        // N s_4_5: branch s_4_4 b56 b5
        if s_4_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_5_0: read-var accdesc.1:struct
        let s_5_0: u32 = fn_state.accdesc._1;
        // C s_5_1: const #5u : u32
        let s_5_1: u32 = 5;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b55 b6
        if s_5_2 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // C s_6_1: const #6u : u32
        let s_6_1: u32 = 6;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: write-var gs#29022 <= s_6_2
        fn_state.gs_29022 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_7_0: read-var gs#29022:u8
        let s_7_0: bool = fn_state.gs_29022;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b54 b8
        if s_7_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#29023 <= s_8_0
        fn_state.gs_29023 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_9_0: read-var gs#29023:u8
        let s_9_0: bool = fn_state.gs_29023;
        // N s_9_1: branch s_9_0 b53 b10
        if s_9_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_10_0: read-var domain:u8
        let s_10_0: u8 = fn_state.domain;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #792u : u32
        let s_10_2: u32 = 792;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // D s_10_5: cmp-eq s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) == (s_10_4));
        // N s_10_6: branch s_10_5 b49 b11
        if s_10_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_12_0: read-var fault.16:struct
        let s_12_0: u32 = fn_state.fault._16;
        // C s_12_1: const #0u : u32
        let s_12_1: u32 = 0;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // N s_12_3: branch s_12_2 b48 b13
        if s_12_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_13_0: read-var accdesc.1:struct
        let s_13_0: u32 = fn_state.accdesc._1;
        // C s_13_1: const #0u : u32
        let s_13_1: u32 = 0;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b44 b14
        if s_13_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#29031 <= s_14_0
        fn_state.gs_29031 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_15_0: read-var gs#29031:u8
        let s_15_0: bool = fn_state.gs_29031;
        // N s_15_1: branch s_15_0 b43 b16
        if s_15_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_16_0: read-var accdesc.1:struct
        let s_16_0: u32 = fn_state.accdesc._1;
        // C s_16_1: const #0u : u32
        let s_16_1: u32 = 0;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // N s_16_3: branch s_16_2 b42 b17
        if s_16_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#29032 <= s_17_0
        fn_state.gs_29032 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_18_0: read-var gs#29032:u8
        let s_18_0: bool = fn_state.gs_29032;
        // N s_18_1: branch s_18_0 b41 b19
        if s_18_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#29033 <= s_19_0
        fn_state.gs_29033 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_20_0: read-var gs#29033:u8
        let s_20_0: bool = fn_state.gs_29033;
        // D s_20_1: write-var gs#29034 <= s_20_0
        fn_state.gs_29034 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_21_0: read-var gs#29034:u8
        let s_21_0: bool = fn_state.gs_29034;
        // N s_21_1: branch s_21_0 b40 b22
        if s_21_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_22_0: read-var walkstate.7:struct
        let s_22_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_22_1: write-var memattrs <= s_22_0
        fn_state.memattrs = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_23_0: read-var regime:u32
        let s_23_0: u32 = fn_state.regime;
        // C s_23_1: const #4u : u32
        let s_23_1: u32 = 4;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // N s_23_3: branch s_23_2 b39 b24
        if s_23_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#29035 <= s_24_0
        fn_state.gs_29035 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_25_0: read-var gs#29035:u8
        let s_25_0: bool = fn_state.gs_29035;
        // N s_25_1: branch s_25_0 b35 b26
        if s_25_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#29036 <= s_26_0
        fn_state.gs_29036 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_27_0: read-var gs#29036:u8
        let s_27_0: bool = fn_state.gs_29036;
        // N s_27_1: branch s_27_0 b34 b28
        if s_27_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#29037 <= s_28_0
        fn_state.gs_29037 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_29_0: read-var gs#29037:u8
        let s_29_0: bool = fn_state.gs_29037;
        // N s_29_1: branch s_29_0 b33 b30
        if s_29_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_30_0: read-var memattrs:struct
        let s_30_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_30_1: call EffectiveShareability(s_30_0)
        let s_30_1: u32 = EffectiveShareability(state, tracer, s_30_0);
        // D s_30_2: write-var memattrs.5 <= s_30_1
        fn_state.memattrs._5 = s_30_1;
        // N s_30_3: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_31_0: read-var walkstate.0:struct
        let s_31_0: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_31_1: read-var walkstate.12:struct
        let s_31_1: u32 = fn_state.walkstate._12;
        // D s_31_2: read-var va:u32
        let s_31_2: u32 = fn_state.va;
        // D s_31_3: call AArch32_SDStageOA(s_31_0, s_31_2, s_31_1)
        let s_31_3: ProductTypeda0231e9dc169f81 = AArch32_SDStageOA(
            state,
            tracer,
            s_31_0,
            s_31_2,
            s_31_1,
        );
        // C s_31_4: const #64s : i
        let s_31_4: i128 = 64;
        // D s_31_5: read-var va:u32
        let s_31_5: u32 = fn_state.va;
        // D s_31_6: cast zx s_31_5 -> bv
        let s_31_6: Bits = Bits::new(s_31_5 as u128, 32u16);
        // D s_31_7: bits-cast zx s_31_6 -> bv length s_31_4
        let s_31_7: Bits = s_31_6.zero_extend(s_31_4);
        // D s_31_8: cast reint s_31_7 -> u64
        let s_31_8: u64 = (s_31_7.value() as u64);
        // D s_31_9: read-var memattrs:struct
        let s_31_9: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_31_10: call CreateAddressDescriptor(s_31_8, s_31_3, s_31_9)
        let s_31_10: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_31_8,
            s_31_3,
            s_31_9,
        );
        // D s_31_11: read-var walkstate.12:struct
        let s_31_11: u32 = fn_state.walkstate._12;
        // D s_31_12: read-var fault:struct
        let s_31_12: ProductType1d757adad216cdef = fn_state.fault;
        // D s_31_13: create-product struct = ["s_31_12", "s_31_10", "s_31_11"]
        let s_31_13: ProductType234df14d4fab6c9d = ProductType234df14d4fab6c9d {
            _0: s_31_12,
            _1: s_31_10,
            _2: s_31_11,
        };
        // D s_31_14: write-var return_value <= s_31_13
        fn_state.return_value = s_31_13;
        // N s_31_15: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_32_0: read-var return_value:struct
        let s_32_0: ProductType234df14d4fab6c9d = fn_state.return_value;
        // N s_32_1: return s_32_0
        return s_32_0;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_33_0: read-var walkstate.7:struct
        let s_33_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_33_1: write-var ga#22420 <= s_33_0
        fn_state.ga_22420 = s_33_0;
        // D s_33_2: read-var ga#22420.5:struct
        let s_33_2: u32 = fn_state.ga_22420._5;
        // D s_33_3: write-var memattrs.5 <= s_33_2
        fn_state.memattrs._5 = s_33_2;
        // N s_33_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_34_0: const #"Apply effective shareability at stage 1" : str
        let s_34_0: &'static str = "Apply effective shareability at stage 1";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // S s_34_2: not s_34_1
        let s_34_2: bool = !s_34_1;
        // D s_34_3: write-var gs#29037 <= s_34_2
        fn_state.gs_29037 = s_34_2;
        // N s_34_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_35_0: read-var accdesc.25:struct
        let s_35_0: u32 = fn_state.accdesc._25;
        // C s_35_1: const #3u : u32
        let s_35_1: u32 = 3;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // C s_35_3: const #432u : u32
        let s_35_3: u32 = 432;
        // D s_35_4: read-reg s_35_3:u8
        let s_35_4: u8 = {
            let value = state.read_register::<u8>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: call ELStateUsingAArch32(s_35_4, s_35_2)
        let s_35_5: bool = ELStateUsingAArch32(state, tracer, s_35_4, s_35_2);
        // N s_35_6: branch s_35_5 b38 b36
        if s_35_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_VM(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_36_1);
        // D s_36_3: write-var ga#22415 <= s_36_2
        fn_state.ga_22415 = s_36_2;
        // N s_36_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_37_0: read-var ga#22415:u8
        let s_37_0: bool = fn_state.ga_22415;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#29036 <= s_37_4
        fn_state.gs_29036 = s_37_4;
        // N s_37_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call HCR_read(s_38_0)
        let s_38_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_38_0);
        // S s_38_2: call _get_HCR_Type_VM(s_38_1)
        let s_38_2: bool = u_get_HCR_Type_VM(state, tracer, s_38_1);
        // D s_38_3: write-var ga#22415 <= s_38_2
        fn_state.ga_22415 = s_38_2;
        // N s_38_4: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_39_0: read-var accdesc.25:struct
        let s_39_0: u32 = fn_state.accdesc._25;
        // D s_39_1: call AArch32_EL2Enabled(s_39_0)
        let s_39_1: bool = AArch32_EL2Enabled(state, tracer, s_39_0);
        // D s_39_2: write-var gs#29035 <= s_39_1
        fn_state.gs_29035 = s_39_1;
        // N s_39_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call NormalNCMemAttr(s_40_0)
        let s_40_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_40_0);
        // D s_40_2: write-var memattrs <= s_40_1
        fn_state.memattrs = s_40_1;
        // D s_40_3: read-var walkstate.7:struct
        let s_40_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_40_4: write-var ga#22408 <= s_40_3
        fn_state.ga_22408 = s_40_3;
        // D s_40_5: read-var ga#22408.7:struct
        let s_40_5: bool = fn_state.ga_22408._7;
        // D s_40_6: write-var memattrs.7 <= s_40_5
        fn_state.memattrs._7 = s_40_5;
        // N s_40_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_41_0: read-var regime:u32
        let s_41_0: u32 = fn_state.regime;
        // D s_41_1: call AArch32_S1DCacheEnabled(s_41_0)
        let s_41_1: bool = AArch32_S1DCacheEnabled(state, tracer, s_41_0);
        // D s_41_2: not s_41_1
        let s_41_2: bool = !s_41_1;
        // D s_41_3: write-var gs#29033 <= s_41_2
        fn_state.gs_29033 = s_41_2;
        // N s_41_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_42_0: read-var walkstate.7:struct
        let s_42_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_42_1: write-var ga#22401 <= s_42_0
        fn_state.ga_22401 = s_42_0;
        // D s_42_2: read-var ga#22401.2:struct
        let s_42_2: u32 = fn_state.ga_22401._2;
        // C s_42_3: const #0u : u32
        let s_42_3: u32 = 0;
        // D s_42_4: cmp-eq s_42_2 s_42_3
        let s_42_4: bool = ((s_42_2) == (s_42_3));
        // D s_42_5: write-var gs#29032 <= s_42_4
        fn_state.gs_29032 = s_42_4;
        // N s_42_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#29034 <= s_43_0
        fn_state.gs_29034 = s_43_0;
        // N s_43_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_44_0: read-var walkstate.7:struct
        let s_44_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_44_1: write-var ga#22395 <= s_44_0
        fn_state.ga_22395 = s_44_0;
        // D s_44_2: read-var ga#22395.2:struct
        let s_44_2: u32 = fn_state.ga_22395._2;
        // C s_44_3: const #1u : u32
        let s_44_3: u32 = 1;
        // D s_44_4: cmp-eq s_44_2 s_44_3
        let s_44_4: bool = ((s_44_2) == (s_44_3));
        // N s_44_5: branch s_44_4 b47 b45
        if s_44_4 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_45_0: read-var regime:u32
        let s_45_0: u32 = fn_state.regime;
        // D s_45_1: call AArch32_S1ICacheEnabled(s_45_0)
        let s_45_1: bool = AArch32_S1ICacheEnabled(state, tracer, s_45_0);
        // D s_45_2: not s_45_1
        let s_45_2: bool = !s_45_1;
        // D s_45_3: write-var gs#29030 <= s_45_2
        fn_state.gs_29030 = s_45_2;
        // N s_45_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_46_0: read-var gs#29030:u8
        let s_46_0: bool = fn_state.gs_29030;
        // D s_46_1: write-var gs#29031 <= s_46_0
        fn_state.gs_29031 = s_46_0;
        // N s_46_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#29030 <= s_47_0
        fn_state.gs_29030 = s_47_0;
        // N s_47_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_48_0: read-var walkstate.3:struct
        let s_48_0: u8 = fn_state.walkstate._3;
        // D s_48_1: write-var fault.4 <= s_48_0
        fn_state.fault._4 = s_48_0;
        // C s_48_2: const #() : ()
        let s_48_2: () = ();
        // S s_48_3: call __UNKNOWN_AddressDescriptor(s_48_2)
        let s_48_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_48_2,
        );
        // D s_48_4: read-var walkstate.12:struct
        let s_48_4: u32 = fn_state.walkstate._12;
        // D s_48_5: read-var fault:struct
        let s_48_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_48_6: create-product struct = ["s_48_5", "s_48_3", "s_48_4"]
        let s_48_6: ProductType234df14d4fab6c9d = ProductType234df14d4fab6c9d {
            _0: s_48_5,
            _1: s_48_3,
            _2: s_48_4,
        };
        // D s_48_7: write-var return_value <= s_48_6
        fn_state.return_value = s_48_6;
        // N s_48_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_49_0: read-var walkstate.9:struct
        let s_49_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_49_1: read-var walkstate.7:struct
        let s_49_1: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_49_2: write-var ga#22383 <= s_49_1
        fn_state.ga_22383 = s_49_1;
        // D s_49_3: read-var ga#22383.2:struct
        let s_49_3: u32 = fn_state.ga_22383._2;
        // D s_49_4: read-var walkstate.0:struct
        let s_49_4: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_49_5: write-var ga#22384 <= s_49_4
        fn_state.ga_22384 = s_49_4;
        // D s_49_6: read-var ga#22384.1:struct
        let s_49_6: u32 = fn_state.ga_22384._1;
        // D s_49_7: read-var regime:u32
        let s_49_7: u32 = fn_state.regime;
        // D s_49_8: read-var accdesc:struct
        let s_49_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_49_9: call AArch32_S1SDHasPermissionsFault(s_49_7, s_49_0, s_49_3, s_49_6, s_49_8)
        let s_49_9: bool = AArch32_S1SDHasPermissionsFault(
            state,
            tracer,
            s_49_7,
            s_49_0,
            s_49_3,
            s_49_6,
            s_49_8,
        );
        // N s_49_10: branch s_49_9 b52 b50
        if s_49_9 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_51_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_52_0: const #5u : u32
        let s_52_0: u32 = 5;
        // D s_52_1: write-var fault.16 <= s_52_0
        fn_state.fault._16 = s_52_0;
        // N s_52_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_53_0: const #4u : u32
        let s_53_0: u32 = 4;
        // D s_53_1: write-var fault.16 <= s_53_0
        fn_state.fault._16 = s_53_0;
        // N s_53_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_54_0: read-var domain:u8
        let s_54_0: u8 = fn_state.domain;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 2u16);
        // C s_54_2: const #784u : u32
        let s_54_2: u32 = 784;
        // D s_54_3: read-reg s_54_2:u8
        let s_54_3: u8 = {
            let value = state.read_register::<u8>(s_54_2 as isize);
            tracer.read_register(s_54_2 as isize, value);
            value
        };
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 2u16);
        // D s_54_5: cmp-eq s_54_1 s_54_4
        let s_54_5: bool = ((s_54_1) == (s_54_4));
        // D s_54_6: write-var gs#29023 <= s_54_5
        fn_state.gs_29023 = s_54_5;
        // N s_54_7: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#29022 <= s_55_0
        fn_state.gs_29022 = s_55_0;
        // N s_55_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_56_0: const #2u : u32
        let s_56_0: u32 = 2;
        // D s_56_1: write-var fault.16 <= s_56_0
        fn_state.fault._16 = s_56_0;
        // N s_56_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_57_0: const #1u : u32
        let s_57_0: u32 = 1;
        // D s_57_1: read-var regime:u32
        let s_57_1: u32 = fn_state.regime;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // N s_57_4: branch s_57_3 b60 b58
        if s_57_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_58_0: const #16456u : u32
        let s_58_0: u32 = 16456;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_SCTLR_Type_nTLSMD(s_58_1)
        let s_58_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_58_1);
        // D s_58_3: write-var ntlsmd <= s_58_2
        fn_state.ntlsmd = s_58_2;
        // N s_58_4: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_59_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_60_0: const #4u : u32
        let s_60_0: u32 = 4;
        // D s_60_1: read-var regime:u32
        let s_60_1: u32 = fn_state.regime;
        // D s_60_2: cmp-eq s_60_0 s_60_1
        let s_60_2: bool = ((s_60_0) == (s_60_1));
        // D s_60_3: not s_60_2
        let s_60_3: bool = !s_60_2;
        // N s_60_4: branch s_60_3 b65 b61
        if s_60_3 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call HaveAArch32EL(s_61_1)
        let s_61_2: bool = HaveAArch32EL(state, tracer, s_61_1);
        // N s_61_3: branch s_61_2 b64 b62
        if s_61_2 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call SCTLR_read__2(s_62_0)
        let s_62_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_62_0);
        // S s_62_2: call _get_SCTLR_Type_nTLSMD(s_62_1)
        let s_62_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_62_1);
        // D s_62_3: write-var ntlsmd <= s_62_2
        fn_state.ntlsmd = s_62_2;
        // N s_62_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_63_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call SCTLR_NS_read(s_64_0)
        let s_64_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_64_0);
        // S s_64_2: call _get_SCTLR_Type_nTLSMD(s_64_1)
        let s_64_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_64_1);
        // D s_64_3: write-var ntlsmd <= s_64_2
        fn_state.ntlsmd = s_64_2;
        // N s_64_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // N s_65_0: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call __UNKNOWN_AddressDescriptor(s_66_0)
        let s_66_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_66_0,
        );
        // C s_66_2: const #() : ()
        let s_66_2: () = ();
        // S s_66_3: call __UNKNOWN_SDFType(s_66_2)
        let s_66_3: u32 = u__UNKNOWN_SDFType(state, tracer, s_66_2);
        // D s_66_4: read-var fault:struct
        let s_66_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_66_5: create-product struct = ["s_66_4", "s_66_1", "s_66_3"]
        let s_66_5: ProductType234df14d4fab6c9d = ProductType234df14d4fab6c9d {
            _0: s_66_4,
            _1: s_66_1,
            _2: s_66_3,
        };
        // D s_66_6: write-var return_value <= s_66_5
        fn_state.return_value = s_66_5;
        // N s_66_7: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType234df14d4fab6c9d {
        // D s_67_0: read-var fault:struct
        let s_67_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_67_1: read-var regime:u32
        let s_67_1: u32 = fn_state.regime;
        // D s_67_2: read-var va:u32
        let s_67_2: u32 = fn_state.va;
        // D s_67_3: read-var aligned:u8
        let s_67_3: bool = fn_state.aligned;
        // D s_67_4: read-var accdesc:struct
        let s_67_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_67_5: call AArch32_S1DisabledOutput(s_67_0, s_67_1, s_67_2, s_67_3, s_67_4)
        let s_67_5: ProductTypedc31059ca7e2391c = AArch32_S1DisabledOutput(
            state,
            tracer,
            s_67_0,
            s_67_1,
            s_67_2,
            s_67_3,
            s_67_4,
        );
        // D s_67_6: write-var ga#22361 <= s_67_5
        fn_state.ga_22361 = s_67_5;
        // D s_67_7: read-var ga#22361.0:struct
        let s_67_7: ProductType1d757adad216cdef = fn_state.ga_22361._0;
        // D s_67_8: read-var ga#22361.1:struct
        let s_67_8: ProductTypece7c66ccb2cab13e = fn_state.ga_22361._1;
        // D s_67_9: write-var fault <= s_67_7
        fn_state.fault = s_67_7;
        // C s_67_10: const #() : ()
        let s_67_10: () = ();
        // S s_67_11: call __UNKNOWN_SDFType(s_67_10)
        let s_67_11: u32 = u__UNKNOWN_SDFType(state, tracer, s_67_10);
        // D s_67_12: read-var fault:struct
        let s_67_12: ProductType1d757adad216cdef = fn_state.fault;
        // D s_67_13: create-product struct = ["s_67_12", "s_67_8", "s_67_11"]
        let s_67_13: ProductType234df14d4fab6c9d = ProductType234df14d4fab6c9d {
            _0: s_67_12,
            _1: s_67_8,
            _2: s_67_11,
        };
        // D s_67_14: write-var return_value <= s_67_13
        fn_state.return_value = s_67_13;
        // N s_67_15: jump b32
        return block_32(state, tracer, fn_state);
    }
}
