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
use HaveAArch32EL::*;
use CreateAccDescAT::*;
use TTBCR_read::*;
use HCR_read::*;
use AArch32_Abort::*;
use AArch32_S2Translate::*;
use PAR_write::*;
use IsExternalAbort__1::*;
use u_get_HCR_Type_VM::*;
use AArch32_EncodePARSD::*;
use u_get_HCR_Type_DC::*;
use u__UNKNOWN_bits::*;
use AArch32_S1TranslateLD::*;
use NoFault__1::*;
use AArch32_S1TranslateSD::*;
use Mk_PAR_Type::*;
use ELUsingAArch32::*;
use TTBCR_NS_read::*;
use SecurityStateAtEL::*;
use u_get_TTBCR_Type_EAE::*;
use EL2Enabled::*;
use AArch32_EncodePARLD::*;
use common::*;
pub fn AArch32_AT<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u32,
    stage_in: u32,
    el: u8,
    ataccess: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30433: bool,
        gs_30447: bool,
        fault: ProductType1d757adad216cdef,
        eae: bool,
        gs_30432: bool,
        addrdesc: ProductTypece7c66ccb2cab13e,
        regime: u32,
        ga_23756: ProductType234df14d4fab6c9d,
        ga_23755: ProductTypedc31059ca7e2391c,
        gs_30443: bool,
        gs_30456: bool,
        gs_30449: bool,
        gs_30454: bool,
        supersection: bool,
        stage: u32,
        gs_30450: bool,
        gs_30436: bool,
        gs_30442: bool,
        ss: u32,
        accdesc: ProductType9878976b5bcce9c9,
        gs_30445: bool,
        ga_23762: ProductTypedc31059ca7e2391c,
        write: bool,
        gs_30444: bool,
        gs_30455: bool,
        gs_30446: bool,
        gs_30448: bool,
        vaddress: u32,
        stage_in: u32,
        el: u8,
        ataccess: u32,
    }
    let fn_state = FunctionState {
        vaddress,
        stage_in,
        el,
        ataccess,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var stage_in:u32
        let s_0_0: u32 = fn_state.stage_in;
        // D s_0_1: write-var stage <= s_0_0
        fn_state.stage = s_0_0;
        // D s_0_2: read-var el:u8
        let s_0_2: u8 = fn_state.el;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #432u : u32
        let s_0_4: u32 = 432;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cmp-eq s_0_3 s_0_6
        let s_0_7: bool = ((s_0_3) == (s_0_6));
        // N s_0_8: branch s_0_7 b73 b1
        if s_0_7 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var stage:u32
        let s_1_0: u32 = fn_state.stage;
        // C s_1_1: const #0u : u32
        let s_1_1: u32 = 0;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b72 b2
        if s_1_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var stage:u32
        let s_2_0: u32 = fn_state.stage;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b71 b3
        if s_2_2 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30432 <= s_3_0
        fn_state.gs_30432 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#30432:u8
        let s_4_0: bool = fn_state.gs_30432;
        // D s_4_1: write-var gs#30433 <= s_4_0
        fn_state.gs_30433 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#30433:u8
        let s_5_0: bool = fn_state.gs_30433;
        // N s_5_1: branch s_5_0 b64 b6
        if s_5_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: write-var regime <= s_6_0
        fn_state.regime = s_6_0;
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: call HaveAArch32EL(s_6_3)
        let s_6_4: bool = HaveAArch32EL(state, tracer, s_6_3);
        // N s_6_5: branch s_6_4 b63 b7
        if s_6_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call TTBCR_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_7_0);
        // S s_7_2: call _get_TTBCR_Type_EAE(s_7_1)
        let s_7_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_7_1);
        // S s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // C s_7_4: const #1u : u8
        let s_7_4: bool = true;
        // C s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 1u16);
        // S s_7_6: cmp-eq s_7_3 s_7_5
        let s_7_6: bool = ((s_7_3) == (s_7_5));
        // D s_7_7: write-var eae <= s_7_6
        fn_state.eae = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: write-var ss <= s_8_0
        fn_state.ss = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var supersection <= s_9_0
        fn_state.supersection = s_9_0;
        // D s_9_2: read-var ataccess:u32
        let s_9_2: u32 = fn_state.ataccess;
        // C s_9_3: const #3u : u32
        let s_9_3: u32 = 3;
        // D s_9_4: cmp-eq s_9_2 s_9_3
        let s_9_4: bool = ((s_9_2) == (s_9_3));
        // N s_9_5: branch s_9_4 b62 b10
        if s_9_4 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ataccess:u32
        let s_10_0: u32 = fn_state.ataccess;
        // C s_10_1: const #1u : u32
        let s_10_1: u32 = 1;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: write-var gs#30442 <= s_10_2
        fn_state.gs_30442 = s_10_2;
        // N s_10_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#30442:u8
        let s_11_0: bool = fn_state.gs_30442;
        // D s_11_1: write-var write <= s_11_0
        fn_state.write = s_11_0;
        // D s_11_2: read-var ataccess:u32
        let s_11_2: u32 = fn_state.ataccess;
        // C s_11_3: const #3u : u32
        let s_11_3: u32 = 3;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b61 b12
        if s_11_4 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ataccess:u32
        let s_12_0: u32 = fn_state.ataccess;
        // C s_12_1: const #2u : u32
        let s_12_1: u32 = 2;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: write-var gs#30443 <= s_12_2
        fn_state.gs_30443 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#30443:u8
        let s_13_0: bool = fn_state.gs_30443;
        // D s_13_1: read-var ss:u32
        let s_13_1: u32 = fn_state.ss;
        // D s_13_2: read-var el:u8
        let s_13_2: u8 = fn_state.el;
        // D s_13_3: read-var write:u8
        let s_13_3: bool = fn_state.write;
        // D s_13_4: call CreateAccDescAT(s_13_1, s_13_2, s_13_3, s_13_0)
        let s_13_4: ProductType9878976b5bcce9c9 = CreateAccDescAT(
            state,
            tracer,
            s_13_1,
            s_13_2,
            s_13_3,
            s_13_0,
        );
        // D s_13_5: write-var accdesc <= s_13_4
        fn_state.accdesc = s_13_4;
        // D s_13_6: read-var accdesc:struct
        let s_13_6: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_13_7: call NoFault__1(s_13_6)
        let s_13_7: ProductType1d757adad216cdef = NoFault__1(state, tracer, s_13_6);
        // D s_13_8: write-var fault <= s_13_7
        fn_state.fault = s_13_7;
        // D s_13_9: read-var eae:u8
        let s_13_9: bool = fn_state.eae;
        // N s_13_10: branch s_13_9 b60 b14
        if s_13_9 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var fault:struct
        let s_14_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_14_1: read-var regime:u32
        let s_14_1: u32 = fn_state.regime;
        // D s_14_2: read-var vaddress:u32
        let s_14_2: u32 = fn_state.vaddress;
        // C s_14_3: const #1u : u8
        let s_14_3: bool = true;
        // D s_14_4: read-var accdesc:struct
        let s_14_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_5: call AArch32_S1TranslateSD(s_14_0, s_14_1, s_14_2, s_14_3, s_14_4)
        let s_14_5: ProductType234df14d4fab6c9d = AArch32_S1TranslateSD(
            state,
            tracer,
            s_14_0,
            s_14_1,
            s_14_2,
            s_14_3,
            s_14_4,
        );
        // D s_14_6: write-var ga#23756 <= s_14_5
        fn_state.ga_23756 = s_14_5;
        // D s_14_7: read-var ga#23756.0:struct
        let s_14_7: ProductType1d757adad216cdef = fn_state.ga_23756._0;
        // D s_14_8: read-var ga#23756.1:struct
        let s_14_8: ProductTypece7c66ccb2cab13e = fn_state.ga_23756._1;
        // D s_14_9: read-var ga#23756.2:struct
        let s_14_9: u32 = fn_state.ga_23756._2;
        // D s_14_10: write-var fault <= s_14_7
        fn_state.fault = s_14_7;
        // D s_14_11: write-var addrdesc <= s_14_8
        fn_state.addrdesc = s_14_8;
        // C s_14_12: const #2u : u32
        let s_14_12: u32 = 2;
        // D s_14_13: cmp-eq s_14_9 s_14_12
        let s_14_13: bool = ((s_14_9) == (s_14_12));
        // N s_14_14: branch s_14_13 b59 b15
        if s_14_13 {
            return block_59(state, tracer, fn_state);
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
        // D s_15_1: write-var supersection <= s_15_0
        fn_state.supersection = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var stage:u32
        let s_17_0: u32 = fn_state.stage;
        // C s_17_1: const #1u : u32
        let s_17_1: u32 = 1;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b58 b18
        if s_17_2 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#30444 <= s_18_0
        fn_state.gs_30444 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#30444:u8
        let s_19_0: bool = fn_state.gs_30444;
        // N s_19_1: branch s_19_0 b57 b20
        if s_19_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var fault.16:struct
        let s_21_0: u32 = fn_state.fault._16;
        // C s_21_1: const #0u : u32
        let s_21_1: u32 = 0;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // N s_21_3: branch s_21_2 b44 b22
        if s_21_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var fault:struct
        let s_23_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_23_1: write-var addrdesc.0 <= s_23_0
        fn_state.addrdesc._0 = s_23_0;
        // D s_23_2: read-var eae:u8
        let s_23_2: bool = fn_state.eae;
        // N s_23_3: branch s_23_2 b43 b24
        if s_23_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var stage:u32
        let s_24_0: u32 = fn_state.stage;
        // C s_24_1: const #1u : u32
        let s_24_1: u32 = 1;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // N s_24_3: branch s_24_2 b39 b25
        if s_24_2 {
            return block_39(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#30446 <= s_25_0
        fn_state.gs_30446 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#30446:u8
        let s_26_0: bool = fn_state.gs_30446;
        // D s_26_1: write-var gs#30447 <= s_26_0
        fn_state.gs_30447 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#30447:u8
        let s_27_0: bool = fn_state.gs_30447;
        // N s_27_1: branch s_27_0 b38 b28
        if s_27_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var stage:u32
        let s_28_0: u32 = fn_state.stage;
        // C s_28_1: const #0u : u32
        let s_28_1: u32 = 0;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // N s_28_3: branch s_28_2 b37 b29
        if s_28_2 {
            return block_37(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#30448 <= s_29_0
        fn_state.gs_30448 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#30448:u8
        let s_30_0: bool = fn_state.gs_30448;
        // N s_30_1: branch s_30_0 b36 b31
        if s_30_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#30449 <= s_31_0
        fn_state.gs_30449 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#30449:u8
        let s_32_0: bool = fn_state.gs_30449;
        // D s_32_1: write-var gs#30450 <= s_32_0
        fn_state.gs_30450 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#30450:u8
        let s_33_0: bool = fn_state.gs_30450;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var addrdesc:struct
        let s_34_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_34_1: read-var supersection:u8
        let s_34_1: bool = fn_state.supersection;
        // D s_34_2: read-var ss:u32
        let s_34_2: u32 = fn_state.ss;
        // D s_34_3: call AArch32_EncodePARSD(s_34_0, s_34_1, s_34_2)
        let s_34_3: () = AArch32_EncodePARSD(state, tracer, s_34_0, s_34_1, s_34_2);
        // N s_34_4: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var addrdesc:struct
        let s_35_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_35_1: read-var ss:u32
        let s_35_1: u32 = fn_state.ss;
        // D s_35_2: call AArch32_EncodePARLD(s_35_0, s_35_1)
        let s_35_2: () = AArch32_EncodePARLD(state, tracer, s_35_0, s_35_1);
        // N s_35_3: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #16975u : u32
        let s_36_0: u32 = 16975;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: cast zx s_36_1 -> bv
        let s_36_2: Bits = Bits::new(s_36_1 as u128, 2u16);
        // C s_36_3: const #432u : u32
        let s_36_3: u32 = 432;
        // D s_36_4: read-reg s_36_3:u8
        let s_36_4: u8 = {
            let value = state.read_register::<u8>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 2u16);
        // D s_36_6: cmp-eq s_36_2 s_36_5
        let s_36_6: bool = ((s_36_2) == (s_36_5));
        // D s_36_7: write-var gs#30449 <= s_36_6
        fn_state.gs_30449 = s_36_6;
        // N s_36_8: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var el:u8
        let s_37_0: u8 = fn_state.el;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 2u16);
        // C s_37_2: const #432u : u32
        let s_37_2: u32 = 432;
        // D s_37_3: read-reg s_37_2:u8
        let s_37_3: u8 = {
            let value = state.read_register::<u8>(s_37_2 as isize);
            tracer.read_register(s_37_2 as isize, value);
            value
        };
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 2u16);
        // D s_37_5: cmp-ne s_37_1 s_37_4
        let s_37_5: bool = ((s_37_1) != (s_37_4));
        // D s_37_6: write-var gs#30448 <= s_37_5
        fn_state.gs_30448 = s_37_5;
        // N s_37_7: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#30450 <= s_38_0
        fn_state.gs_30450 = s_38_0;
        // N s_38_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call HCR_read(s_39_0)
        let s_39_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_39_0);
        // S s_39_2: call _get_HCR_Type_VM(s_39_1)
        let s_39_2: bool = u_get_HCR_Type_VM(state, tracer, s_39_1);
        // S s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // S s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // N s_39_7: branch s_39_6 b42 b40
        if s_39_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HCR_read(s_40_0)
        let s_40_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_40_0);
        // S s_40_2: call _get_HCR_Type_DC(s_40_1)
        let s_40_2: bool = u_get_HCR_Type_DC(state, tracer, s_40_1);
        // S s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // C s_40_4: const #1u : u8
        let s_40_4: bool = true;
        // C s_40_5: cast zx s_40_4 -> bv
        let s_40_5: Bits = Bits::new(s_40_4 as u128, 1u16);
        // S s_40_6: cmp-eq s_40_3 s_40_5
        let s_40_6: bool = ((s_40_3) == (s_40_5));
        // D s_40_7: write-var gs#30445 <= s_40_6
        fn_state.gs_30445 = s_40_6;
        // N s_40_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#30445:u8
        let s_41_0: bool = fn_state.gs_30445;
        // D s_41_1: write-var gs#30446 <= s_41_0
        fn_state.gs_30446 = s_41_0;
        // N s_41_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#30445 <= s_42_0
        fn_state.gs_30445 = s_42_0;
        // N s_42_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#30447 <= s_43_0
        fn_state.gs_30447 = s_43_0;
        // N s_43_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var fault:struct
        let s_44_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_44_1: call IsExternalAbort__1(s_44_0)
        let s_44_1: bool = IsExternalAbort__1(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b56 b45
        if s_44_1 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #16975u : u32
        let s_45_0: u32 = 16975;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: cast zx s_45_1 -> bv
        let s_45_2: Bits = Bits::new(s_45_1 as u128, 2u16);
        // C s_45_3: const #440u : u32
        let s_45_3: u32 = 440;
        // D s_45_4: read-reg s_45_3:u8
        let s_45_4: u8 = {
            let value = state.read_register::<u8>(s_45_3 as isize);
            tracer.read_register(s_45_3 as isize, value);
            value
        };
        // D s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 2u16);
        // D s_45_6: cmp-eq s_45_2 s_45_5
        let s_45_6: bool = ((s_45_2) == (s_45_5));
        // N s_45_7: branch s_45_6 b55 b46
        if s_45_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#30454 <= s_46_0
        fn_state.gs_30454 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#30454:u8
        let s_47_0: bool = fn_state.gs_30454;
        // N s_47_1: branch s_47_0 b54 b48
        if s_47_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#30455 <= s_48_0
        fn_state.gs_30455 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#30455:u8
        let s_49_0: bool = fn_state.gs_30455;
        // D s_49_1: write-var gs#30456 <= s_49_0
        fn_state.gs_30456 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#30456:u8
        let s_50_0: bool = fn_state.gs_30456;
        // N s_50_1: branch s_50_0 b53 b51
        if s_50_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // C s_53_1: cast zx s_53_0 -> i
        let s_53_1: i128 = (i128::try_from(s_53_0).unwrap());
        // S s_53_2: call __UNKNOWN_bits(s_53_1)
        let s_53_2: Bits = u__UNKNOWN_bits(state, tracer, s_53_1);
        // S s_53_3: cast reint s_53_2 -> u64
        let s_53_3: u64 = (s_53_2.value() as u64);
        // S s_53_4: call Mk_PAR_Type(s_53_3)
        let s_53_4: ProductType5c790c8ef59cc8b2 = Mk_PAR_Type(state, tracer, s_53_3);
        // S s_53_5: call PAR_write(s_53_4)
        let s_53_5: () = PAR_write(state, tracer, s_53_4);
        // D s_53_6: read-var vaddress:u32
        let s_53_6: u32 = fn_state.vaddress;
        // D s_53_7: read-var fault:struct
        let s_53_7: ProductType1d757adad216cdef = fn_state.fault;
        // D s_53_8: call AArch32_Abort(s_53_6, s_53_7)
        let s_53_8: () = AArch32_Abort(state, tracer, s_53_6, s_53_7);
        // N s_53_9: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var fault.14:struct
        let s_54_0: bool = fn_state.fault._14;
        // D s_54_1: write-var gs#30455 <= s_54_0
        fn_state.gs_30455 = s_54_0;
        // N s_54_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EL2Enabled(s_55_0)
        let s_55_1: bool = EL2Enabled(state, tracer, s_55_0);
        // D s_55_2: write-var gs#30454 <= s_55_1
        fn_state.gs_30454 = s_55_1;
        // N s_55_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#30456 <= s_56_0
        fn_state.gs_30456 = s_56_0;
        // N s_56_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // D s_57_1: create-sum enum = 0:"s_57_0"
        let s_57_1: SumTypebf36e919d71ba1d6 = SumTypebf36e919d71ba1d6::_0(s_57_0);
        // D s_57_2: read-var fault:struct
        let s_57_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_57_3: read-var addrdesc:struct
        let s_57_3: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // D s_57_5: read-var accdesc:struct
        let s_57_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_57_6: call AArch32_S2Translate(s_57_2, s_57_3, s_57_1, s_57_4, s_57_5)
        let s_57_6: ProductTypedc31059ca7e2391c = AArch32_S2Translate(
            state,
            tracer,
            s_57_2,
            s_57_3,
            s_57_1,
            s_57_4,
            s_57_5,
        );
        // D s_57_7: write-var ga#23762 <= s_57_6
        fn_state.ga_23762 = s_57_6;
        // D s_57_8: read-var ga#23762.0:struct
        let s_57_8: ProductType1d757adad216cdef = fn_state.ga_23762._0;
        // D s_57_9: read-var ga#23762.1:struct
        let s_57_9: ProductTypece7c66ccb2cab13e = fn_state.ga_23762._1;
        // D s_57_10: write-var fault <= s_57_8
        fn_state.fault = s_57_8;
        // D s_57_11: write-var addrdesc <= s_57_9
        fn_state.addrdesc = s_57_9;
        // N s_57_12: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var fault.16:struct
        let s_58_0: u32 = fn_state.fault._16;
        // C s_58_1: const #0u : u32
        let s_58_1: u32 = 0;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: write-var gs#30444 <= s_58_2
        fn_state.gs_30444 = s_58_2;
        // N s_58_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #1u : u8
        let s_59_0: bool = true;
        // D s_59_1: write-var supersection <= s_59_0
        fn_state.supersection = s_59_0;
        // N s_59_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var fault:struct
        let s_60_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_60_1: read-var regime:u32
        let s_60_1: u32 = fn_state.regime;
        // D s_60_2: read-var vaddress:u32
        let s_60_2: u32 = fn_state.vaddress;
        // C s_60_3: const #1u : u8
        let s_60_3: bool = true;
        // D s_60_4: read-var accdesc:struct
        let s_60_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_60_5: call AArch32_S1TranslateLD(s_60_0, s_60_1, s_60_2, s_60_3, s_60_4)
        let s_60_5: ProductTypedc31059ca7e2391c = AArch32_S1TranslateLD(
            state,
            tracer,
            s_60_0,
            s_60_1,
            s_60_2,
            s_60_3,
            s_60_4,
        );
        // D s_60_6: write-var ga#23755 <= s_60_5
        fn_state.ga_23755 = s_60_5;
        // D s_60_7: read-var ga#23755.0:struct
        let s_60_7: ProductType1d757adad216cdef = fn_state.ga_23755._0;
        // D s_60_8: read-var ga#23755.1:struct
        let s_60_8: ProductTypece7c66ccb2cab13e = fn_state.ga_23755._1;
        // D s_60_9: write-var fault <= s_60_7
        fn_state.fault = s_60_7;
        // D s_60_10: write-var addrdesc <= s_60_8
        fn_state.addrdesc = s_60_8;
        // N s_60_11: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#30443 <= s_61_0
        fn_state.gs_30443 = s_61_0;
        // N s_61_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#30442 <= s_62_0
        fn_state.gs_30442 = s_62_0;
        // N s_62_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call TTBCR_NS_read(s_63_0)
        let s_63_1: ProductType700c18a878c5601b = TTBCR_NS_read(state, tracer, s_63_0);
        // S s_63_2: call _get_TTBCR_Type_EAE(s_63_1)
        let s_63_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_63_1);
        // S s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // C s_63_4: const #1u : u8
        let s_63_4: bool = true;
        // C s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 1u16);
        // S s_63_6: cmp-eq s_63_3 s_63_5
        let s_63_6: bool = ((s_63_3) == (s_63_5));
        // D s_63_7: write-var eae <= s_63_6
        fn_state.eae = s_63_6;
        // N s_63_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u32
        let s_64_0: u32 = 0;
        // D s_64_1: write-var stage <= s_64_0
        fn_state.stage = s_64_0;
        // C s_64_2: const #16975u : u32
        let s_64_2: u32 = 16975;
        // D s_64_3: read-reg s_64_2:u8
        let s_64_3: u8 = {
            let value = state.read_register::<u8>(s_64_2 as isize);
            tracer.read_register(s_64_2 as isize, value);
            value
        };
        // D s_64_4: call SecurityStateAtEL(s_64_3)
        let s_64_4: u32 = SecurityStateAtEL(state, tracer, s_64_3);
        // D s_64_5: write-var ss <= s_64_4
        fn_state.ss = s_64_4;
        // D s_64_6: read-var ss:u32
        let s_64_6: u32 = fn_state.ss;
        // C s_64_7: const #3u : u32
        let s_64_7: u32 = 3;
        // D s_64_8: cmp-eq s_64_6 s_64_7
        let s_64_8: bool = ((s_64_6) == (s_64_7));
        // N s_64_9: branch s_64_8 b70 b65
        if s_64_8 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#30436 <= s_65_0
        fn_state.gs_30436 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#30436:u8
        let s_66_0: bool = fn_state.gs_30436;
        // N s_66_1: branch s_66_0 b69 b67
        if s_66_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #4u : u32
        let s_67_0: u32 = 4;
        // D s_67_1: write-var regime <= s_67_0
        fn_state.regime = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call TTBCR_read(s_68_0)
        let s_68_1: ProductType700c18a878c5601b = TTBCR_read(state, tracer, s_68_0);
        // S s_68_2: call _get_TTBCR_Type_EAE(s_68_1)
        let s_68_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_68_1);
        // S s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // C s_68_4: const #1u : u8
        let s_68_4: bool = true;
        // C s_68_5: cast zx s_68_4 -> bv
        let s_68_5: Bits = Bits::new(s_68_4 as u128, 1u16);
        // S s_68_6: cmp-eq s_68_3 s_68_5
        let s_68_6: bool = ((s_68_3) == (s_68_5));
        // D s_68_7: write-var eae <= s_68_6
        fn_state.eae = s_68_6;
        // N s_68_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u32
        let s_69_0: u32 = 1;
        // D s_69_1: write-var regime <= s_69_0
        fn_state.regime = s_69_0;
        // N s_69_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call ELUsingAArch32(s_70_1)
        let s_70_2: bool = ELUsingAArch32(state, tracer, s_70_1);
        // D s_70_3: write-var gs#30436 <= s_70_2
        fn_state.gs_30436 = s_70_2;
        // N s_70_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #432u : u32
        let s_71_0: u32 = 432;
        // D s_71_1: read-reg s_71_0:u8
        let s_71_1: u8 = {
            let value = state.read_register::<u8>(s_71_0 as isize);
            tracer.read_register(s_71_0 as isize, value);
            value
        };
        // C s_71_2: const #2u : u8
        let s_71_2: u8 = 2;
        // D s_71_3: cmp-lt s_71_1 s_71_2
        let s_71_3: bool = ((s_71_1) < (s_71_2));
        // D s_71_4: not s_71_3
        let s_71_4: bool = !s_71_3;
        // D s_71_5: write-var gs#30432 <= s_71_4
        fn_state.gs_30432 = s_71_4;
        // N s_71_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#30433 <= s_72_0
        fn_state.gs_30433 = s_72_0;
        // N s_72_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #2u : u32
        let s_73_0: u32 = 2;
        // D s_73_1: write-var regime <= s_73_0
        fn_state.regime = s_73_0;
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // D s_73_3: write-var eae <= s_73_2
        fn_state.eae = s_73_2;
        // C s_73_4: const #0u : u32
        let s_73_4: u32 = 0;
        // D s_73_5: write-var ss <= s_73_4
        fn_state.ss = s_73_4;
        // N s_73_6: jump b9
        return block_9(state, tracer, fn_state);
    }
}
