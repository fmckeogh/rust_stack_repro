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
use AArch32_S1WalkLD::*;
use AArch32_S1Enabled::*;
use SetInGuardedPage::*;
use StageOA::*;
use HCR_read::*;
use AArch32_EL2Enabled::*;
use AArch32_S1LDHasPermissionsFault::*;
use AArch32_S1DCacheEnabled::*;
use u__IMPDEF_boolean::*;
use AArch32_S1HasAlignmentFault::*;
use EffectiveShareability::*;
use CreateAddressDescriptor::*;
use u_get_HCR_Type_VM::*;
use AArch32_VAIsOutOfRange::*;
use ELStateUsingAArch32::*;
use AArch32_S1ICacheEnabled::*;
use u__UNKNOWN_AddressDescriptor::*;
use NormalNCMemAttr::*;
use AArch32_S1DisabledOutput::*;
use u_get_HCR_EL2_Type_VM::*;
use AArch32_GetS1TTWParams::*;
use common::*;
pub fn AArch32_S1TranslateLD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    va: u32,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        gs_28411: bool,
        ga_22033: ProductTypef170cab34335b70c,
        ga_22010: ProductTypeda0231e9dc169f81,
        ga_22020: ProductTypef170cab34335b70c,
        fault: ProductType1d757adad216cdef,
        return_value: ProductTypedc31059ca7e2391c,
        gs_28415: bool,
        walkparams: ProductTypeef284266e139aee2,
        gs_28416: bool,
        ga_22026: ProductTypef170cab34335b70c,
        gs_28412: bool,
        gs_28414: bool,
        ga_22009: ProductTypef170cab34335b70c,
        gs_28409: bool,
        gs_28410: bool,
        walkstate: ProductType96e7acababe246a1,
        gs_28413: bool,
        ga_22045: ProductTypef170cab34335b70c,
        ga_22001: ProductType201519a0f62623dc,
        memattrs: ProductTypef170cab34335b70c,
        ga_22040: bool,
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
    ) -> ProductTypedc31059ca7e2391c {
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
        // N s_0_6: branch s_0_5 b47 b1
        if s_0_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // D s_1_1: read-var va:u32
        let s_1_1: u32 = fn_state.va;
        // D s_1_2: call AArch32_GetS1TTWParams(s_1_0, s_1_1)
        let s_1_2: ProductTypeef284266e139aee2 = AArch32_GetS1TTWParams(
            state,
            tracer,
            s_1_0,
            s_1_1,
        );
        // D s_1_3: write-var walkparams <= s_1_2
        fn_state.walkparams = s_1_2;
        // D s_1_4: read-var regime:u32
        let s_1_4: u32 = fn_state.regime;
        // D s_1_5: read-var walkparams:struct
        let s_1_5: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_1_6: read-var va:u32
        let s_1_6: u32 = fn_state.va;
        // D s_1_7: call AArch32_VAIsOutOfRange(s_1_4, s_1_5, s_1_6)
        let s_1_7: bool = AArch32_VAIsOutOfRange(state, tracer, s_1_4, s_1_5, s_1_6);
        // N s_1_8: branch s_1_7 b46 b2
        if s_1_7 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var fault:struct
        let s_2_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_2_1: read-var regime:u32
        let s_2_1: u32 = fn_state.regime;
        // D s_2_2: read-var walkparams:struct
        let s_2_2: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_2_3: read-var accdesc:struct
        let s_2_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_2_4: read-var va:u32
        let s_2_4: u32 = fn_state.va;
        // D s_2_5: call AArch32_S1WalkLD(s_2_0, s_2_1, s_2_2, s_2_3, s_2_4)
        let s_2_5: ProductType201519a0f62623dc = AArch32_S1WalkLD(
            state,
            tracer,
            s_2_0,
            s_2_1,
            s_2_2,
            s_2_3,
            s_2_4,
        );
        // D s_2_6: write-var ga#22001 <= s_2_5
        fn_state.ga_22001 = s_2_5;
        // D s_2_7: read-var ga#22001.0:struct
        let s_2_7: ProductType1d757adad216cdef = fn_state.ga_22001._0;
        // D s_2_8: read-var ga#22001.1:struct
        let s_2_8: ProductType96e7acababe246a1 = fn_state.ga_22001._1;
        // D s_2_9: write-var fault <= s_2_7
        fn_state.fault = s_2_7;
        // D s_2_10: write-var walkstate <= s_2_8
        fn_state.walkstate = s_2_8;
        // D s_2_11: read-var fault.16:struct
        let s_2_11: u32 = fn_state.fault._16;
        // C s_2_12: const #0u : u32
        let s_2_12: u32 = 0;
        // D s_2_13: cmp-eq s_2_11 s_2_12
        let s_2_13: bool = ((s_2_11) == (s_2_12));
        // N s_2_14: branch s_2_13 b45 b3
        if s_2_13 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // S s_3_1: call SetInGuardedPage(s_3_0)
        let s_3_1: () = SetInGuardedPage(state, tracer, s_3_0);
        // D s_3_2: read-var walkparams.21:struct
        let s_3_2: bool = fn_state.walkparams._21;
        // D s_3_3: read-var walkstate.7:struct
        let s_3_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_3_4: read-var accdesc:struct
        let s_3_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_5: read-var aligned:u8
        let s_3_5: bool = fn_state.aligned;
        // D s_3_6: call AArch32_S1HasAlignmentFault(s_3_4, s_3_5, s_3_2, s_3_3)
        let s_3_6: bool = AArch32_S1HasAlignmentFault(
            state,
            tracer,
            s_3_4,
            s_3_5,
            s_3_2,
            s_3_3,
        );
        // N s_3_7: branch s_3_6 b44 b4
        if s_3_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_4_0: read-var walkstate.9:struct
        let s_4_0: ProductTypebf05c51f33174538 = fn_state.walkstate._9;
        // D s_4_1: read-var walkstate.7:struct
        let s_4_1: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_4_2: write-var ga#22009 <= s_4_1
        fn_state.ga_22009 = s_4_1;
        // D s_4_3: read-var ga#22009.2:struct
        let s_4_3: u32 = fn_state.ga_22009._2;
        // D s_4_4: read-var walkstate.0:struct
        let s_4_4: ProductTypeda0231e9dc169f81 = fn_state.walkstate._0;
        // D s_4_5: write-var ga#22010 <= s_4_4
        fn_state.ga_22010 = s_4_4;
        // D s_4_6: read-var ga#22010.1:struct
        let s_4_6: u32 = fn_state.ga_22010._1;
        // D s_4_7: read-var regime:u32
        let s_4_7: u32 = fn_state.regime;
        // D s_4_8: read-var walkparams:struct
        let s_4_8: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_4_9: read-var accdesc:struct
        let s_4_9: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_10: call AArch32_S1LDHasPermissionsFault(s_4_7, s_4_8, s_4_0, s_4_3, s_4_6, s_4_9)
        let s_4_10: bool = AArch32_S1LDHasPermissionsFault(
            state,
            tracer,
            s_4_7,
            s_4_8,
            s_4_0,
            s_4_3,
            s_4_6,
            s_4_9,
        );
        // N s_4_11: branch s_4_10 b43 b5
        if s_4_10 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_6_0: read-var fault.16:struct
        let s_6_0: u32 = fn_state.fault._16;
        // C s_6_1: const #0u : u32
        let s_6_1: u32 = 0;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b42 b7
        if s_6_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_7_0: read-var accdesc.1:struct
        let s_7_0: u32 = fn_state.accdesc._1;
        // C s_7_1: const #0u : u32
        let s_7_1: u32 = 0;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b38 b8
        if s_7_2 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#28410 <= s_8_0
        fn_state.gs_28410 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_9_0: read-var gs#28410:u8
        let s_9_0: bool = fn_state.gs_28410;
        // N s_9_1: branch s_9_0 b37 b10
        if s_9_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_10_0: read-var accdesc.1:struct
        let s_10_0: u32 = fn_state.accdesc._1;
        // C s_10_1: const #0u : u32
        let s_10_1: u32 = 0;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b36 b11
        if s_10_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#28411 <= s_11_0
        fn_state.gs_28411 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_12_0: read-var gs#28411:u8
        let s_12_0: bool = fn_state.gs_28411;
        // N s_12_1: branch s_12_0 b35 b13
        if s_12_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#28412 <= s_13_0
        fn_state.gs_28412 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_14_0: read-var gs#28412:u8
        let s_14_0: bool = fn_state.gs_28412;
        // D s_14_1: write-var gs#28413 <= s_14_0
        fn_state.gs_28413 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_15_0: read-var gs#28413:u8
        let s_15_0: bool = fn_state.gs_28413;
        // N s_15_1: branch s_15_0 b34 b16
        if s_15_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_16_0: read-var walkstate.7:struct
        let s_16_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_16_1: write-var memattrs <= s_16_0
        fn_state.memattrs = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_17_0: read-var regime:u32
        let s_17_0: u32 = fn_state.regime;
        // C s_17_1: const #4u : u32
        let s_17_1: u32 = 4;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b33 b18
        if s_17_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#28414 <= s_18_0
        fn_state.gs_28414 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_19_0: read-var gs#28414:u8
        let s_19_0: bool = fn_state.gs_28414;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#28415 <= s_20_0
        fn_state.gs_28415 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_21_0: read-var gs#28415:u8
        let s_21_0: bool = fn_state.gs_28415;
        // N s_21_1: branch s_21_0 b28 b22
        if s_21_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#28416 <= s_22_0
        fn_state.gs_28416 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_23_0: read-var gs#28416:u8
        let s_23_0: bool = fn_state.gs_28416;
        // N s_23_1: branch s_23_0 b27 b24
        if s_23_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_24_0: read-var memattrs:struct
        let s_24_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_24_1: call EffectiveShareability(s_24_0)
        let s_24_1: u32 = EffectiveShareability(state, tracer, s_24_0);
        // D s_24_2: write-var memattrs.5 <= s_24_1
        fn_state.memattrs._5 = s_24_1;
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_25_0: const #64s : i
        let s_25_0: i128 = 64;
        // D s_25_1: read-var va:u32
        let s_25_1: u32 = fn_state.va;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 32u16);
        // D s_25_3: bits-cast zx s_25_2 -> bv length s_25_0
        let s_25_3: Bits = s_25_2.zero_extend(s_25_0);
        // D s_25_4: cast reint s_25_3 -> u64
        let s_25_4: u64 = (s_25_3.value() as u64);
        // D s_25_5: read-var walkparams.3:struct
        let s_25_5: bool = fn_state.walkparams._3;
        // D s_25_6: read-var walkparams.36:struct
        let s_25_6: u32 = fn_state.walkparams._36;
        // D s_25_7: read-var walkstate:struct
        let s_25_7: ProductType96e7acababe246a1 = fn_state.walkstate;
        // D s_25_8: call StageOA(s_25_4, s_25_5, s_25_6, s_25_7)
        let s_25_8: ProductTypeda0231e9dc169f81 = StageOA(
            state,
            tracer,
            s_25_4,
            s_25_5,
            s_25_6,
            s_25_7,
        );
        // C s_25_9: const #64s : i
        let s_25_9: i128 = 64;
        // D s_25_10: read-var va:u32
        let s_25_10: u32 = fn_state.va;
        // D s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 32u16);
        // D s_25_12: bits-cast zx s_25_11 -> bv length s_25_9
        let s_25_12: Bits = s_25_11.zero_extend(s_25_9);
        // D s_25_13: cast reint s_25_12 -> u64
        let s_25_13: u64 = (s_25_12.value() as u64);
        // D s_25_14: read-var memattrs:struct
        let s_25_14: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_25_15: call CreateAddressDescriptor(s_25_13, s_25_8, s_25_14)
        let s_25_15: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_25_13,
            s_25_8,
            s_25_14,
        );
        // D s_25_16: read-var fault:struct
        let s_25_16: ProductType1d757adad216cdef = fn_state.fault;
        // D s_25_17: create-product struct = ["s_25_16", "s_25_15"]
        let s_25_17: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_25_16,
            _1: s_25_15,
        };
        // D s_25_18: write-var return_value <= s_25_17
        fn_state.return_value = s_25_17;
        // N s_25_19: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_26_0: read-var return_value:struct
        let s_26_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_26_1: return s_26_0
        return s_26_0;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_27_0: read-var walkstate.7:struct
        let s_27_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_27_1: write-var ga#22045 <= s_27_0
        fn_state.ga_22045 = s_27_0;
        // D s_27_2: read-var ga#22045.5:struct
        let s_27_2: u32 = fn_state.ga_22045._5;
        // D s_27_3: write-var memattrs.5 <= s_27_2
        fn_state.memattrs._5 = s_27_2;
        // N s_27_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_28_0: const #"Apply effective shareability at stage 1" : str
        let s_28_0: &'static str = "Apply effective shareability at stage 1";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // S s_28_2: not s_28_1
        let s_28_2: bool = !s_28_1;
        // D s_28_3: write-var gs#28416 <= s_28_2
        fn_state.gs_28416 = s_28_2;
        // N s_28_4: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_29_0: read-var accdesc.25:struct
        let s_29_0: u32 = fn_state.accdesc._25;
        // C s_29_1: const #3u : u32
        let s_29_1: u32 = 3;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // C s_29_3: const #432u : u32
        let s_29_3: u32 = 432;
        // D s_29_4: read-reg s_29_3:u8
        let s_29_4: u8 = {
            let value = state.read_register::<u8>(s_29_3 as isize);
            tracer.read_register(s_29_3 as isize, value);
            value
        };
        // D s_29_5: call ELStateUsingAArch32(s_29_4, s_29_2)
        let s_29_5: bool = ELStateUsingAArch32(state, tracer, s_29_4, s_29_2);
        // N s_29_6: branch s_29_5 b32 b30
        if s_29_5 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_30_0: const #102552u : u32
        let s_30_0: u32 = 102552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_HCR_EL2_Type_VM(s_30_1)
        let s_30_2: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_30_1);
        // D s_30_3: write-var ga#22040 <= s_30_2
        fn_state.ga_22040 = s_30_2;
        // N s_30_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_31_0: read-var ga#22040:u8
        let s_31_0: bool = fn_state.ga_22040;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#28415 <= s_31_4
        fn_state.gs_28415 = s_31_4;
        // N s_31_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call HCR_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_32_0);
        // S s_32_2: call _get_HCR_Type_VM(s_32_1)
        let s_32_2: bool = u_get_HCR_Type_VM(state, tracer, s_32_1);
        // D s_32_3: write-var ga#22040 <= s_32_2
        fn_state.ga_22040 = s_32_2;
        // N s_32_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_33_0: read-var accdesc.25:struct
        let s_33_0: u32 = fn_state.accdesc._25;
        // D s_33_1: call AArch32_EL2Enabled(s_33_0)
        let s_33_1: bool = AArch32_EL2Enabled(state, tracer, s_33_0);
        // D s_33_2: write-var gs#28414 <= s_33_1
        fn_state.gs_28414 = s_33_1;
        // N s_33_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call NormalNCMemAttr(s_34_0)
        let s_34_1: ProductTypef170cab34335b70c = NormalNCMemAttr(state, tracer, s_34_0);
        // D s_34_2: write-var memattrs <= s_34_1
        fn_state.memattrs = s_34_1;
        // D s_34_3: read-var walkstate.7:struct
        let s_34_3: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_34_4: write-var ga#22033 <= s_34_3
        fn_state.ga_22033 = s_34_3;
        // D s_34_5: read-var ga#22033.7:struct
        let s_34_5: bool = fn_state.ga_22033._7;
        // D s_34_6: write-var memattrs.7 <= s_34_5
        fn_state.memattrs._7 = s_34_5;
        // N s_34_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_35_0: read-var regime:u32
        let s_35_0: u32 = fn_state.regime;
        // D s_35_1: call AArch32_S1DCacheEnabled(s_35_0)
        let s_35_1: bool = AArch32_S1DCacheEnabled(state, tracer, s_35_0);
        // D s_35_2: not s_35_1
        let s_35_2: bool = !s_35_1;
        // D s_35_3: write-var gs#28412 <= s_35_2
        fn_state.gs_28412 = s_35_2;
        // N s_35_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_36_0: read-var walkstate.7:struct
        let s_36_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_36_1: write-var ga#22026 <= s_36_0
        fn_state.ga_22026 = s_36_0;
        // D s_36_2: read-var ga#22026.2:struct
        let s_36_2: u32 = fn_state.ga_22026._2;
        // C s_36_3: const #0u : u32
        let s_36_3: u32 = 0;
        // D s_36_4: cmp-eq s_36_2 s_36_3
        let s_36_4: bool = ((s_36_2) == (s_36_3));
        // D s_36_5: write-var gs#28411 <= s_36_4
        fn_state.gs_28411 = s_36_4;
        // N s_36_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#28413 <= s_37_0
        fn_state.gs_28413 = s_37_0;
        // N s_37_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_38_0: read-var walkstate.7:struct
        let s_38_0: ProductTypef170cab34335b70c = fn_state.walkstate._7;
        // D s_38_1: write-var ga#22020 <= s_38_0
        fn_state.ga_22020 = s_38_0;
        // D s_38_2: read-var ga#22020.2:struct
        let s_38_2: u32 = fn_state.ga_22020._2;
        // C s_38_3: const #1u : u32
        let s_38_3: u32 = 1;
        // D s_38_4: cmp-eq s_38_2 s_38_3
        let s_38_4: bool = ((s_38_2) == (s_38_3));
        // N s_38_5: branch s_38_4 b41 b39
        if s_38_4 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_39_0: read-var regime:u32
        let s_39_0: u32 = fn_state.regime;
        // D s_39_1: call AArch32_S1ICacheEnabled(s_39_0)
        let s_39_1: bool = AArch32_S1ICacheEnabled(state, tracer, s_39_0);
        // D s_39_2: not s_39_1
        let s_39_2: bool = !s_39_1;
        // D s_39_3: write-var gs#28409 <= s_39_2
        fn_state.gs_28409 = s_39_2;
        // N s_39_4: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_40_0: read-var gs#28409:u8
        let s_40_0: bool = fn_state.gs_28409;
        // D s_40_1: write-var gs#28410 <= s_40_0
        fn_state.gs_28410 = s_40_0;
        // N s_40_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#28409 <= s_41_0
        fn_state.gs_28409 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call __UNKNOWN_AddressDescriptor(s_42_0)
        let s_42_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_42_0,
        );
        // D s_42_2: read-var fault:struct
        let s_42_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_42_3: create-product struct = ["s_42_2", "s_42_1"]
        let s_42_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_42_2,
            _1: s_42_1,
        };
        // D s_42_4: write-var return_value <= s_42_3
        fn_state.return_value = s_42_3;
        // N s_42_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_43_0: const #5u : u32
        let s_43_0: u32 = 5;
        // D s_43_1: write-var fault.16 <= s_43_0
        fn_state.fault._16 = s_43_0;
        // N s_43_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_44_0: const #2u : u32
        let s_44_0: u32 = 2;
        // D s_44_1: write-var fault.16 <= s_44_0
        fn_state.fault._16 = s_44_0;
        // N s_44_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call __UNKNOWN_AddressDescriptor(s_45_0)
        let s_45_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_45_0,
        );
        // D s_45_2: read-var fault:struct
        let s_45_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_45_3: create-product struct = ["s_45_2", "s_45_1"]
        let s_45_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_45_2,
            _1: s_45_1,
        };
        // D s_45_4: write-var return_value <= s_45_3
        fn_state.return_value = s_45_3;
        // N s_45_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_46_0: const #1s : i
        let s_46_0: i128 = 1;
        // D s_46_1: write-var fault.9 <= s_46_0
        fn_state.fault._9 = s_46_0;
        // C s_46_2: const #6u : u32
        let s_46_2: u32 = 6;
        // D s_46_3: write-var fault.16 <= s_46_2
        fn_state.fault._16 = s_46_2;
        // C s_46_4: const #() : ()
        let s_46_4: () = ();
        // S s_46_5: call __UNKNOWN_AddressDescriptor(s_46_4)
        let s_46_5: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_46_4,
        );
        // D s_46_6: read-var fault:struct
        let s_46_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_46_7: create-product struct = ["s_46_6", "s_46_5"]
        let s_46_7: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_46_6,
            _1: s_46_5,
        };
        // D s_46_8: write-var return_value <= s_46_7
        fn_state.return_value = s_46_7;
        // N s_46_9: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_47_0: read-var fault:struct
        let s_47_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_47_1: read-var regime:u32
        let s_47_1: u32 = fn_state.regime;
        // D s_47_2: read-var va:u32
        let s_47_2: u32 = fn_state.va;
        // D s_47_3: read-var aligned:u8
        let s_47_3: bool = fn_state.aligned;
        // D s_47_4: read-var accdesc:struct
        let s_47_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_47_5: call AArch32_S1DisabledOutput(s_47_0, s_47_1, s_47_2, s_47_3, s_47_4)
        let s_47_5: ProductTypedc31059ca7e2391c = AArch32_S1DisabledOutput(
            state,
            tracer,
            s_47_0,
            s_47_1,
            s_47_2,
            s_47_3,
            s_47_4,
        );
        // D s_47_6: write-var return_value <= s_47_5
        fn_state.return_value = s_47_5;
        // N s_47_7: jump b26
        return block_26(state, tracer, fn_state);
    }
}
