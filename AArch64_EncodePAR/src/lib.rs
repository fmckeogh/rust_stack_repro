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
use PAREncodeShareability::*;
use u__IMPDEF_bit::*;
use PAR_EL1_write::*;
use HaveRME::*;
use AArch64_PARFaultStatus::*;
use u_update_PAR_EL1_Type_SH::*;
use Zeros::*;
use u_get_PAR_EL1_Type_D128::*;
use u_update_PAR_EL1_Type_D128::*;
use ReportedPARShareability::*;
use u_update_PAR_EL1_Type_NSE::*;
use PAR_EL1_read::*;
use u_update_PAR_EL1_Type_AssuredOnly::*;
use u_update_PAR_EL1_Type_ATTR::*;
use AArch64_isPARFormatD128::*;
use u_update_PAR_EL1_Type_DirtyBit::*;
use ReportedPARAttrs::*;
use u_update_PAR_EL1_Type_PTW::*;
use u_update_PAR_EL1_Type_S::*;
use u__UNKNOWN_bit::*;
use u_update_PAR_EL1_Type_FST::*;
use Mk_PAR_EL1_Type::*;
use SecurityStateForRegime::*;
use EncodePARAttrs::*;
use u_update_PAR_EL1_Type_F::*;
use u__IMPDEF_bits::*;
use IsFault::*;
use u_update_PAR_EL1_Type_NS::*;
use u_update_PAR_EL1_Type_Overlay::*;
use u_update_PAR_EL1_Type_TopLevel::*;
use common::*;
pub fn AArch64_EncodePAR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    is_ATS1Ex: bool,
    addrdesc: ProductTypece7c66ccb2cab13e,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_22643: ProductType1d757adad216cdef,
        ga_22641: bool,
        gs_29196: bool,
        ga_22646: bool,
        ga_22665: bool,
        ga_22566: bool,
        ga_22583: bool,
        ga_22655: ProductType782ac6922b48c20d,
        ga_22656: bool,
        ga_22612: ProductType782ac6922b48c20d,
        ga_22613: ProductTypeda0231e9dc169f81,
        ga_22682: ProductTypeda0231e9dc169f81,
        ga_22599: bool,
        ga_22664: ProductType782ac6922b48c20d,
        ga_22619: ProductType782ac6922b48c20d,
        ga_22669: ProductType782ac6922b48c20d,
        ga_22620: ProductTypeda0231e9dc169f81,
        paspace: u32,
        ga_22591: ProductType782ac6922b48c20d,
        ga_22672: ProductType782ac6922b48c20d,
        ga_22662: ProductType1d757adad216cdef,
        ga_22653: ProductType1d757adad216cdef,
        ga_22582: ProductType782ac6922b48c20d,
        ga_22667: ProductType1d757adad216cdef,
        ga_22645: ProductType782ac6922b48c20d,
        ga_22631: ProductType782ac6922b48c20d,
        ga_22565: ProductType782ac6922b48c20d,
        ga_22651: bool,
        ga_22598: ProductType782ac6922b48c20d,
        ga_22676: ProductType782ac6922b48c20d,
        ga_22640: ProductType782ac6922b48c20d,
        ga_22670: bool,
        ga_22650: ProductType782ac6922b48c20d,
        ga_22638: ProductType1d757adad216cdef,
        ga_22648: ProductType1d757adad216cdef,
        regime: u32,
        is_ATS1Ex: bool,
        addrdesc: ProductTypece7c66ccb2cab13e,
    }
    let fn_state = FunctionState {
        regime,
        is_ATS1Ex,
        addrdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #128s : i
        let s_0_0: i128 = 128;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u128
        let s_0_2: u128 = (s_0_1.value() as u128);
        // S s_0_3: call Mk_PAR_EL1_Type(s_0_2)
        let s_0_3: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(state, tracer, s_0_2);
        // S s_0_4: call PAR_EL1_write(s_0_3)
        let s_0_4: () = PAR_EL1_write(state, tracer, s_0_3);
        // D s_0_5: read-var addrdesc.3:struct
        let s_0_5: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_0_6: write-var ga#22682 <= s_0_5
        fn_state.ga_22682 = s_0_5;
        // D s_0_7: read-var ga#22682.1:struct
        let s_0_7: u32 = fn_state.ga_22682._1;
        // D s_0_8: write-var paspace <= s_0_7
        fn_state.paspace = s_0_7;
        // D s_0_9: read-var regime:u32
        let s_0_9: u32 = fn_state.regime;
        // D s_0_10: read-var is_ATS1Ex:u8
        let s_0_10: bool = fn_state.is_ATS1Ex;
        // D s_0_11: call AArch64_isPARFormatD128(s_0_9, s_0_10)
        let s_0_11: bool = AArch64_isPARFormatD128(state, tracer, s_0_9, s_0_10);
        // N s_0_12: branch s_0_11 b60 b1
        if s_0_11 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call PAR_EL1_read(s_1_0)
        let s_1_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_1_0);
        // C s_1_2: const #0u : u8
        let s_1_2: bool = false;
        // S s_1_3: call _update_PAR_EL1_Type_D128(s_1_1, s_1_2)
        let s_1_3: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_D128(
            state,
            tracer,
            s_1_1,
            s_1_2,
        );
        // S s_1_4: call PAR_EL1_write(s_1_3)
        let s_1_4: () = PAR_EL1_write(state, tracer, s_1_3);
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var addrdesc:struct
        let s_2_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_2_1: call IsFault(s_2_0)
        let s_2_1: bool = IsFault(state, tracer, s_2_0);
        // D s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b22 b3
        if s_2_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PAR_EL1_read(s_3_0)
        let s_3_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_3_0);
        // C s_3_2: const #1u : u8
        let s_3_2: bool = true;
        // S s_3_3: call _update_PAR_EL1_Type_F(s_3_1, s_3_2)
        let s_3_3: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_F(
            state,
            tracer,
            s_3_1,
            s_3_2,
        );
        // S s_3_4: call PAR_EL1_write(s_3_3)
        let s_3_4: () = PAR_EL1_write(state, tracer, s_3_3);
        // C s_3_5: const #() : ()
        let s_3_5: () = ();
        // S s_3_6: call PAR_EL1_read(s_3_5)
        let s_3_6: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_3_5);
        // D s_3_7: write-var ga#22640 <= s_3_6
        fn_state.ga_22640 = s_3_6;
        // D s_3_8: read-var addrdesc.0:struct
        let s_3_8: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_3_9: write-var ga#22638 <= s_3_8
        fn_state.ga_22638 = s_3_8;
        // D s_3_10: read-var ga#22638.3:struct
        let s_3_10: bool = fn_state.ga_22638._3;
        // N s_3_11: branch s_3_10 b21 b4
        if s_3_10 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var ga#22641 <= s_4_0
        fn_state.ga_22641 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#22640:struct
        let s_5_0: ProductType782ac6922b48c20d = fn_state.ga_22640;
        // D s_5_1: read-var ga#22641:u8
        let s_5_1: bool = fn_state.ga_22641;
        // D s_5_2: call _update_PAR_EL1_Type_DirtyBit(s_5_0, s_5_1)
        let s_5_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_DirtyBit(
            state,
            tracer,
            s_5_0,
            s_5_1,
        );
        // D s_5_3: call PAR_EL1_write(s_5_2)
        let s_5_3: () = PAR_EL1_write(state, tracer, s_5_2);
        // C s_5_4: const #() : ()
        let s_5_4: () = ();
        // S s_5_5: call PAR_EL1_read(s_5_4)
        let s_5_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_4);
        // D s_5_6: write-var ga#22645 <= s_5_5
        fn_state.ga_22645 = s_5_5;
        // D s_5_7: read-var addrdesc.0:struct
        let s_5_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_5_8: write-var ga#22643 <= s_5_7
        fn_state.ga_22643 = s_5_7;
        // D s_5_9: read-var ga#22643.11:struct
        let s_5_9: bool = fn_state.ga_22643._11;
        // N s_5_10: branch s_5_9 b20 b6
        if s_5_9 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var ga#22646 <= s_6_0
        fn_state.ga_22646 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#22645:struct
        let s_7_0: ProductType782ac6922b48c20d = fn_state.ga_22645;
        // D s_7_1: read-var ga#22646:u8
        let s_7_1: bool = fn_state.ga_22646;
        // D s_7_2: call _update_PAR_EL1_Type_Overlay(s_7_0, s_7_1)
        let s_7_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_Overlay(
            state,
            tracer,
            s_7_0,
            s_7_1,
        );
        // D s_7_3: call PAR_EL1_write(s_7_2)
        let s_7_3: () = PAR_EL1_write(state, tracer, s_7_2);
        // C s_7_4: const #() : ()
        let s_7_4: () = ();
        // S s_7_5: call PAR_EL1_read(s_7_4)
        let s_7_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_7_4);
        // D s_7_6: write-var ga#22650 <= s_7_5
        fn_state.ga_22650 = s_7_5;
        // D s_7_7: read-var addrdesc.0:struct
        let s_7_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_7_8: write-var ga#22648 <= s_7_7
        fn_state.ga_22648 = s_7_7;
        // D s_7_9: read-var ga#22648.18:struct
        let s_7_9: bool = fn_state.ga_22648._18;
        // N s_7_10: branch s_7_9 b19 b8
        if s_7_9 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var ga#22651 <= s_8_0
        fn_state.ga_22651 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#22650:struct
        let s_9_0: ProductType782ac6922b48c20d = fn_state.ga_22650;
        // D s_9_1: read-var ga#22651:u8
        let s_9_1: bool = fn_state.ga_22651;
        // D s_9_2: call _update_PAR_EL1_Type_TopLevel(s_9_0, s_9_1)
        let s_9_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_TopLevel(
            state,
            tracer,
            s_9_0,
            s_9_1,
        );
        // D s_9_3: call PAR_EL1_write(s_9_2)
        let s_9_3: () = PAR_EL1_write(state, tracer, s_9_2);
        // C s_9_4: const #() : ()
        let s_9_4: () = ();
        // S s_9_5: call PAR_EL1_read(s_9_4)
        let s_9_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_9_4);
        // D s_9_6: write-var ga#22655 <= s_9_5
        fn_state.ga_22655 = s_9_5;
        // D s_9_7: read-var addrdesc.0:struct
        let s_9_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_9_8: write-var ga#22653 <= s_9_7
        fn_state.ga_22653 = s_9_7;
        // D s_9_9: read-var ga#22653.1:struct
        let s_9_9: bool = fn_state.ga_22653._1;
        // N s_9_10: branch s_9_9 b18 b10
        if s_9_9 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var ga#22656 <= s_10_0
        fn_state.ga_22656 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#22655:struct
        let s_11_0: ProductType782ac6922b48c20d = fn_state.ga_22655;
        // D s_11_1: read-var ga#22656:u8
        let s_11_1: bool = fn_state.ga_22656;
        // D s_11_2: call _update_PAR_EL1_Type_AssuredOnly(s_11_0, s_11_1)
        let s_11_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_AssuredOnly(
            state,
            tracer,
            s_11_0,
            s_11_1,
        );
        // D s_11_3: call PAR_EL1_write(s_11_2)
        let s_11_3: () = PAR_EL1_write(state, tracer, s_11_2);
        // C s_11_4: const #() : ()
        let s_11_4: () = ();
        // S s_11_5: call PAR_EL1_read(s_11_4)
        let s_11_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_11_4);
        // D s_11_6: read-var addrdesc.0:struct
        let s_11_6: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_11_7: call AArch64_PARFaultStatus(s_11_6)
        let s_11_7: u8 = AArch64_PARFaultStatus(state, tracer, s_11_6);
        // D s_11_8: call _update_PAR_EL1_Type_FST(s_11_5, s_11_7)
        let s_11_8: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_FST(
            state,
            tracer,
            s_11_5,
            s_11_7,
        );
        // D s_11_9: call PAR_EL1_write(s_11_8)
        let s_11_9: () = PAR_EL1_write(state, tracer, s_11_8);
        // C s_11_10: const #() : ()
        let s_11_10: () = ();
        // S s_11_11: call PAR_EL1_read(s_11_10)
        let s_11_11: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_11_10);
        // D s_11_12: write-var ga#22664 <= s_11_11
        fn_state.ga_22664 = s_11_11;
        // D s_11_13: read-var addrdesc.0:struct
        let s_11_13: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_11_14: write-var ga#22662 <= s_11_13
        fn_state.ga_22662 = s_11_13;
        // D s_11_15: read-var ga#22662.14:struct
        let s_11_15: bool = fn_state.ga_22662._14;
        // N s_11_16: branch s_11_15 b17 b12
        if s_11_15 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var ga#22665 <= s_12_0
        fn_state.ga_22665 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#22664:struct
        let s_13_0: ProductType782ac6922b48c20d = fn_state.ga_22664;
        // D s_13_1: read-var ga#22665:u8
        let s_13_1: bool = fn_state.ga_22665;
        // D s_13_2: call _update_PAR_EL1_Type_PTW(s_13_0, s_13_1)
        let s_13_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_PTW(
            state,
            tracer,
            s_13_0,
            s_13_1,
        );
        // D s_13_3: call PAR_EL1_write(s_13_2)
        let s_13_3: () = PAR_EL1_write(state, tracer, s_13_2);
        // C s_13_4: const #() : ()
        let s_13_4: () = ();
        // S s_13_5: call PAR_EL1_read(s_13_4)
        let s_13_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_13_4);
        // D s_13_6: write-var ga#22669 <= s_13_5
        fn_state.ga_22669 = s_13_5;
        // D s_13_7: read-var addrdesc.0:struct
        let s_13_7: ProductType1d757adad216cdef = fn_state.addrdesc._0;
        // D s_13_8: write-var ga#22667 <= s_13_7
        fn_state.ga_22667 = s_13_7;
        // D s_13_9: read-var ga#22667.15:struct
        let s_13_9: bool = fn_state.ga_22667._15;
        // N s_13_10: branch s_13_9 b16 b14
        if s_13_9 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var ga#22670 <= s_14_0
        fn_state.ga_22670 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#22669:struct
        let s_15_0: ProductType782ac6922b48c20d = fn_state.ga_22669;
        // D s_15_1: read-var ga#22670:u8
        let s_15_1: bool = fn_state.ga_22670;
        // D s_15_2: call _update_PAR_EL1_Type_S(s_15_0, s_15_1)
        let s_15_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_S(
            state,
            tracer,
            s_15_0,
            s_15_1,
        );
        // D s_15_3: call PAR_EL1_write(s_15_2)
        let s_15_3: () = PAR_EL1_write(state, tracer, s_15_2);
        // C s_15_4: const #() : ()
        let s_15_4: () = ();
        // S s_15_5: call PAR_EL1_read(s_15_4)
        let s_15_5: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_15_4);
        // D s_15_6: write-var ga#22672 <= s_15_5
        fn_state.ga_22672 = s_15_5;
        // D s_15_7: read-var ga#22672.0:struct
        let s_15_7: u128 = fn_state.ga_22672._0;
        // C s_15_8: const #11s : i
        let s_15_8: i128 = 11;
        // D s_15_9: cast zx s_15_7 -> bv
        let s_15_9: Bits = Bits::new(s_15_7 as u128, 128u16);
        // C s_15_10: const #1u : u8
        let s_15_10: bool = true;
        // C s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 1u16);
        // C s_15_12: const #0s : i
        let s_15_12: i128 = 0;
        // C s_15_13: const #1u : u64
        let s_15_13: u64 = 1;
        // C s_15_14: cast zx s_15_13 -> bv
        let s_15_14: Bits = Bits::new(s_15_13 as u128, 64u16);
        // C s_15_15: lsl s_15_14 s_15_12
        let s_15_15: Bits = s_15_14 << s_15_12;
        // C s_15_16: sub s_15_15 s_15_14
        let s_15_16: Bits = ((s_15_15) - (s_15_14));
        // C s_15_17: and s_15_11 s_15_16
        let s_15_17: Bits = ((s_15_11) & (s_15_16));
        // C s_15_18: lsl s_15_17 s_15_8
        let s_15_18: Bits = s_15_17 << s_15_8;
        // C s_15_19: lsl s_15_16 s_15_8
        let s_15_19: Bits = s_15_16 << s_15_8;
        // C s_15_20: cmpl s_15_19
        let s_15_20: Bits = !s_15_19;
        // D s_15_21: and s_15_9 s_15_20
        let s_15_21: Bits = ((s_15_9) & (s_15_20));
        // D s_15_22: or s_15_21 s_15_18
        let s_15_22: Bits = ((s_15_21) | (s_15_18));
        // D s_15_23: cast reint s_15_22 -> u128
        let s_15_23: u128 = (s_15_22.value() as u128);
        // D s_15_24: call Mk_PAR_EL1_Type(s_15_23)
        let s_15_24: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_15_23,
        );
        // D s_15_25: call PAR_EL1_write(s_15_24)
        let s_15_25: () = PAR_EL1_write(state, tracer, s_15_24);
        // C s_15_26: const #() : ()
        let s_15_26: () = ();
        // S s_15_27: call PAR_EL1_read(s_15_26)
        let s_15_27: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_15_26);
        // D s_15_28: write-var ga#22676 <= s_15_27
        fn_state.ga_22676 = s_15_27;
        // D s_15_29: read-var ga#22676.0:struct
        let s_15_29: u128 = fn_state.ga_22676._0;
        // C s_15_30: const #16s : i64
        let s_15_30: i64 = 16;
        // C s_15_31: cast zx s_15_30 -> i
        let s_15_31: i128 = (i128::try_from(s_15_30).unwrap());
        // C s_15_32: const #"Faulting PAR" : str
        let s_15_32: &'static str = "Faulting PAR";
        // S s_15_33: call __IMPDEF_bits(s_15_31, s_15_32)
        let s_15_33: Bits = u__IMPDEF_bits(state, tracer, s_15_31, s_15_32);
        // S s_15_34: cast reint s_15_33 -> u16
        let s_15_34: u16 = (s_15_33.value() as u16);
        // C s_15_35: const #48s : i
        let s_15_35: i128 = 48;
        // D s_15_36: cast zx s_15_29 -> bv
        let s_15_36: Bits = Bits::new(s_15_29 as u128, 128u16);
        // S s_15_37: cast zx s_15_34 -> bv
        let s_15_37: Bits = Bits::new(s_15_34 as u128, 16u16);
        // C s_15_38: const #15s : i
        let s_15_38: i128 = 15;
        // C s_15_39: const #1u : u64
        let s_15_39: u64 = 1;
        // C s_15_40: cast zx s_15_39 -> bv
        let s_15_40: Bits = Bits::new(s_15_39 as u128, 64u16);
        // C s_15_41: lsl s_15_40 s_15_38
        let s_15_41: Bits = s_15_40 << s_15_38;
        // C s_15_42: sub s_15_41 s_15_40
        let s_15_42: Bits = ((s_15_41) - (s_15_40));
        // S s_15_43: and s_15_37 s_15_42
        let s_15_43: Bits = ((s_15_37) & (s_15_42));
        // S s_15_44: lsl s_15_43 s_15_35
        let s_15_44: Bits = s_15_43 << s_15_35;
        // C s_15_45: lsl s_15_42 s_15_35
        let s_15_45: Bits = s_15_42 << s_15_35;
        // C s_15_46: cmpl s_15_45
        let s_15_46: Bits = !s_15_45;
        // D s_15_47: and s_15_36 s_15_46
        let s_15_47: Bits = ((s_15_36) & (s_15_46));
        // D s_15_48: or s_15_47 s_15_44
        let s_15_48: Bits = ((s_15_47) | (s_15_44));
        // D s_15_49: cast reint s_15_48 -> u128
        let s_15_49: u128 = (s_15_48.value() as u128);
        // D s_15_50: call Mk_PAR_EL1_Type(s_15_49)
        let s_15_50: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_15_49,
        );
        // D s_15_51: call PAR_EL1_write(s_15_50)
        let s_15_51: () = PAR_EL1_write(state, tracer, s_15_50);
        // N s_15_52: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var ga#22670 <= s_16_0
        fn_state.ga_22670 = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var ga#22665 <= s_17_0
        fn_state.ga_22665 = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var ga#22656 <= s_18_0
        fn_state.ga_22656 = s_18_0;
        // N s_18_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var ga#22651 <= s_19_0
        fn_state.ga_22651 = s_19_0;
        // N s_19_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var ga#22646 <= s_20_0
        fn_state.ga_22646 = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var ga#22641 <= s_21_0
        fn_state.ga_22641 = s_21_0;
        // N s_21_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call PAR_EL1_read(s_22_0)
        let s_22_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_22_0);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // S s_22_3: call _update_PAR_EL1_Type_F(s_22_1, s_22_2)
        let s_22_3: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_F(
            state,
            tracer,
            s_22_1,
            s_22_2,
        );
        // S s_22_4: call PAR_EL1_write(s_22_3)
        let s_22_4: () = PAR_EL1_write(state, tracer, s_22_3);
        // C s_22_5: const #() : ()
        let s_22_5: () = ();
        // S s_22_6: call HaveRME(s_22_5)
        let s_22_6: bool = HaveRME(state, tracer, s_22_5);
        // N s_22_7: branch s_22_6 b33 b23
        if s_22_6 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call PAR_EL1_read(s_23_0)
        let s_23_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_23_0);
        // D s_23_2: write-var ga#22591 <= s_23_1
        fn_state.ga_22591 = s_23_1;
        // D s_23_3: read-var ga#22591.0:struct
        let s_23_3: u128 = fn_state.ga_22591._0;
        // C s_23_4: const #11s : i
        let s_23_4: i128 = 11;
        // D s_23_5: cast zx s_23_3 -> bv
        let s_23_5: Bits = Bits::new(s_23_3 as u128, 128u16);
        // C s_23_6: const #1u : u8
        let s_23_6: bool = true;
        // C s_23_7: cast zx s_23_6 -> bv
        let s_23_7: Bits = Bits::new(s_23_6 as u128, 1u16);
        // C s_23_8: const #0s : i
        let s_23_8: i128 = 0;
        // C s_23_9: const #1u : u64
        let s_23_9: u64 = 1;
        // C s_23_10: cast zx s_23_9 -> bv
        let s_23_10: Bits = Bits::new(s_23_9 as u128, 64u16);
        // C s_23_11: lsl s_23_10 s_23_8
        let s_23_11: Bits = s_23_10 << s_23_8;
        // C s_23_12: sub s_23_11 s_23_10
        let s_23_12: Bits = ((s_23_11) - (s_23_10));
        // C s_23_13: and s_23_7 s_23_12
        let s_23_13: Bits = ((s_23_7) & (s_23_12));
        // C s_23_14: lsl s_23_13 s_23_4
        let s_23_14: Bits = s_23_13 << s_23_4;
        // C s_23_15: lsl s_23_12 s_23_4
        let s_23_15: Bits = s_23_12 << s_23_4;
        // C s_23_16: cmpl s_23_15
        let s_23_16: Bits = !s_23_15;
        // D s_23_17: and s_23_5 s_23_16
        let s_23_17: Bits = ((s_23_5) & (s_23_16));
        // D s_23_18: or s_23_17 s_23_14
        let s_23_18: Bits = ((s_23_17) | (s_23_14));
        // D s_23_19: cast reint s_23_18 -> u128
        let s_23_19: u128 = (s_23_18.value() as u128);
        // D s_23_20: call Mk_PAR_EL1_Type(s_23_19)
        let s_23_20: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_23_19,
        );
        // D s_23_21: call PAR_EL1_write(s_23_20)
        let s_23_21: () = PAR_EL1_write(state, tracer, s_23_20);
        // D s_23_22: read-var regime:u32
        let s_23_22: u32 = fn_state.regime;
        // D s_23_23: call SecurityStateForRegime(s_23_22)
        let s_23_23: u32 = SecurityStateForRegime(state, tracer, s_23_22);
        // C s_23_24: const #3u : u32
        let s_23_24: u32 = 3;
        // D s_23_25: cmp-eq s_23_23 s_23_24
        let s_23_25: bool = ((s_23_23) == (s_23_24));
        // N s_23_26: branch s_23_25 b29 b24
        if s_23_25 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call PAR_EL1_read(s_24_0)
        let s_24_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_24_0);
        // C s_24_2: const #() : ()
        let s_24_2: () = ();
        // S s_24_3: call __UNKNOWN_bit(s_24_2)
        let s_24_3: bool = u__UNKNOWN_bit(state, tracer, s_24_2);
        // S s_24_4: call _update_PAR_EL1_Type_NS(s_24_1, s_24_3)
        let s_24_4: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_24_1,
            s_24_3,
        );
        // S s_24_5: call PAR_EL1_write(s_24_4)
        let s_24_5: () = PAR_EL1_write(state, tracer, s_24_4);
        // N s_24_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call PAR_EL1_read(s_25_0)
        let s_25_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_25_0);
        // D s_25_2: read-var addrdesc.2:struct
        let s_25_2: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_25_3: call PAREncodeShareability(s_25_2)
        let s_25_3: u8 = PAREncodeShareability(state, tracer, s_25_2);
        // D s_25_4: call ReportedPARShareability(s_25_3)
        let s_25_4: u8 = ReportedPARShareability(state, tracer, s_25_3);
        // D s_25_5: call _update_PAR_EL1_Type_SH(s_25_1, s_25_4)
        let s_25_5: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_SH(
            state,
            tracer,
            s_25_1,
            s_25_4,
        );
        // D s_25_6: call PAR_EL1_write(s_25_5)
        let s_25_6: () = PAR_EL1_write(state, tracer, s_25_5);
        // C s_25_7: const #() : ()
        let s_25_7: () = ();
        // S s_25_8: call PAR_EL1_read(s_25_7)
        let s_25_8: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_25_7);
        // S s_25_9: call _get_PAR_EL1_Type_D128(s_25_8)
        let s_25_9: bool = u_get_PAR_EL1_Type_D128(state, tracer, s_25_8);
        // S s_25_10: cast zx s_25_9 -> bv
        let s_25_10: Bits = Bits::new(s_25_9 as u128, 1u16);
        // C s_25_11: const #1u : u8
        let s_25_11: bool = true;
        // C s_25_12: cast zx s_25_11 -> bv
        let s_25_12: Bits = Bits::new(s_25_11 as u128, 1u16);
        // S s_25_13: cmp-eq s_25_10 s_25_12
        let s_25_13: bool = ((s_25_10) == (s_25_12));
        // N s_25_14: branch s_25_13 b28 b26
        if s_25_13 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call PAR_EL1_read(s_26_0)
        let s_26_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_26_0);
        // D s_26_2: write-var ga#22619 <= s_26_1
        fn_state.ga_22619 = s_26_1;
        // D s_26_3: read-var ga#22619.0:struct
        let s_26_3: u128 = fn_state.ga_22619._0;
        // D s_26_4: read-var addrdesc.3:struct
        let s_26_4: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_26_5: write-var ga#22620 <= s_26_4
        fn_state.ga_22620 = s_26_4;
        // D s_26_6: read-var ga#22620.0:struct
        let s_26_6: u64 = fn_state.ga_22620._0;
        // C s_26_7: const #12s : i
        let s_26_7: i128 = 12;
        // D s_26_8: cast zx s_26_6 -> bv
        let s_26_8: Bits = Bits::new(s_26_6 as u128, 56u16);
        // C s_26_9: const #1s : i64
        let s_26_9: i64 = 1;
        // C s_26_10: cast zx s_26_9 -> i
        let s_26_10: i128 = (i128::try_from(s_26_9).unwrap());
        // C s_26_11: const #43s : i
        let s_26_11: i128 = 43;
        // C s_26_12: add s_26_11 s_26_10
        let s_26_12: i128 = (s_26_11 + s_26_10);
        // D s_26_13: bit-extract s_26_8 s_26_7 s_26_12
        let s_26_13: Bits = (Bits::new(
            ((s_26_8) >> (s_26_7)).value(),
            u16::try_from(s_26_12).unwrap(),
        ));
        // D s_26_14: cast reint s_26_13 -> u44
        let s_26_14: u64 = (s_26_13.value() as u64);
        // C s_26_15: const #12s : i
        let s_26_15: i128 = 12;
        // D s_26_16: cast zx s_26_3 -> bv
        let s_26_16: Bits = Bits::new(s_26_3 as u128, 128u16);
        // D s_26_17: cast zx s_26_14 -> bv
        let s_26_17: Bits = Bits::new(s_26_14 as u128, 44u16);
        // C s_26_18: const #43s : i
        let s_26_18: i128 = 43;
        // C s_26_19: const #1u : u64
        let s_26_19: u64 = 1;
        // C s_26_20: cast zx s_26_19 -> bv
        let s_26_20: Bits = Bits::new(s_26_19 as u128, 64u16);
        // C s_26_21: lsl s_26_20 s_26_18
        let s_26_21: Bits = s_26_20 << s_26_18;
        // C s_26_22: sub s_26_21 s_26_20
        let s_26_22: Bits = ((s_26_21) - (s_26_20));
        // D s_26_23: and s_26_17 s_26_22
        let s_26_23: Bits = ((s_26_17) & (s_26_22));
        // D s_26_24: lsl s_26_23 s_26_15
        let s_26_24: Bits = s_26_23 << s_26_15;
        // C s_26_25: lsl s_26_22 s_26_15
        let s_26_25: Bits = s_26_22 << s_26_15;
        // C s_26_26: cmpl s_26_25
        let s_26_26: Bits = !s_26_25;
        // D s_26_27: and s_26_16 s_26_26
        let s_26_27: Bits = ((s_26_16) & (s_26_26));
        // D s_26_28: or s_26_27 s_26_24
        let s_26_28: Bits = ((s_26_27) | (s_26_24));
        // D s_26_29: cast reint s_26_28 -> u128
        let s_26_29: u128 = (s_26_28.value() as u128);
        // D s_26_30: call Mk_PAR_EL1_Type(s_26_29)
        let s_26_30: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_26_29,
        );
        // D s_26_31: call PAR_EL1_write(s_26_30)
        let s_26_31: () = PAR_EL1_write(state, tracer, s_26_30);
        // N s_26_32: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call PAR_EL1_read(s_27_0)
        let s_27_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_27_0);
        // D s_27_2: read-var addrdesc.2:struct
        let s_27_2: ProductTypef170cab34335b70c = fn_state.addrdesc._2;
        // D s_27_3: call EncodePARAttrs(s_27_2)
        let s_27_3: u8 = EncodePARAttrs(state, tracer, s_27_2);
        // D s_27_4: call ReportedPARAttrs(s_27_3)
        let s_27_4: u8 = ReportedPARAttrs(state, tracer, s_27_3);
        // D s_27_5: call _update_PAR_EL1_Type_ATTR(s_27_1, s_27_4)
        let s_27_5: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_ATTR(
            state,
            tracer,
            s_27_1,
            s_27_4,
        );
        // D s_27_6: call PAR_EL1_write(s_27_5)
        let s_27_6: () = PAR_EL1_write(state, tracer, s_27_5);
        // C s_27_7: const #() : ()
        let s_27_7: () = ();
        // S s_27_8: call PAR_EL1_read(s_27_7)
        let s_27_8: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_27_7);
        // D s_27_9: write-var ga#22631 <= s_27_8
        fn_state.ga_22631 = s_27_8;
        // D s_27_10: read-var ga#22631.0:struct
        let s_27_10: u128 = fn_state.ga_22631._0;
        // C s_27_11: const #"Non-Faulting PAR" : str
        let s_27_11: &'static str = "Non-Faulting PAR";
        // S s_27_12: call __IMPDEF_bit(s_27_11)
        let s_27_12: bool = u__IMPDEF_bit(state, tracer, s_27_11);
        // C s_27_13: const #10s : i
        let s_27_13: i128 = 10;
        // D s_27_14: cast zx s_27_10 -> bv
        let s_27_14: Bits = Bits::new(s_27_10 as u128, 128u16);
        // S s_27_15: cast zx s_27_12 -> bv
        let s_27_15: Bits = Bits::new(s_27_12 as u128, 1u16);
        // C s_27_16: const #0s : i
        let s_27_16: i128 = 0;
        // C s_27_17: const #1u : u64
        let s_27_17: u64 = 1;
        // C s_27_18: cast zx s_27_17 -> bv
        let s_27_18: Bits = Bits::new(s_27_17 as u128, 64u16);
        // C s_27_19: lsl s_27_18 s_27_16
        let s_27_19: Bits = s_27_18 << s_27_16;
        // C s_27_20: sub s_27_19 s_27_18
        let s_27_20: Bits = ((s_27_19) - (s_27_18));
        // S s_27_21: and s_27_15 s_27_20
        let s_27_21: Bits = ((s_27_15) & (s_27_20));
        // S s_27_22: lsl s_27_21 s_27_13
        let s_27_22: Bits = s_27_21 << s_27_13;
        // C s_27_23: lsl s_27_20 s_27_13
        let s_27_23: Bits = s_27_20 << s_27_13;
        // C s_27_24: cmpl s_27_23
        let s_27_24: Bits = !s_27_23;
        // D s_27_25: and s_27_14 s_27_24
        let s_27_25: Bits = ((s_27_14) & (s_27_24));
        // D s_27_26: or s_27_25 s_27_22
        let s_27_26: Bits = ((s_27_25) | (s_27_22));
        // D s_27_27: cast reint s_27_26 -> u128
        let s_27_27: u128 = (s_27_26.value() as u128);
        // D s_27_28: call Mk_PAR_EL1_Type(s_27_27)
        let s_27_28: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_27_27,
        );
        // D s_27_29: call PAR_EL1_write(s_27_28)
        let s_27_29: () = PAR_EL1_write(state, tracer, s_27_28);
        // N s_27_30: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call PAR_EL1_read(s_28_0)
        let s_28_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_28_0);
        // D s_28_2: write-var ga#22612 <= s_28_1
        fn_state.ga_22612 = s_28_1;
        // D s_28_3: read-var ga#22612.0:struct
        let s_28_3: u128 = fn_state.ga_22612._0;
        // D s_28_4: read-var addrdesc.3:struct
        let s_28_4: ProductTypeda0231e9dc169f81 = fn_state.addrdesc._3;
        // D s_28_5: write-var ga#22613 <= s_28_4
        fn_state.ga_22613 = s_28_4;
        // D s_28_6: read-var ga#22613.0:struct
        let s_28_6: u64 = fn_state.ga_22613._0;
        // C s_28_7: const #12s : i
        let s_28_7: i128 = 12;
        // D s_28_8: cast zx s_28_6 -> bv
        let s_28_8: Bits = Bits::new(s_28_6 as u128, 56u16);
        // C s_28_9: const #1s : i64
        let s_28_9: i64 = 1;
        // C s_28_10: cast zx s_28_9 -> i
        let s_28_10: i128 = (i128::try_from(s_28_9).unwrap());
        // C s_28_11: const #43s : i
        let s_28_11: i128 = 43;
        // C s_28_12: add s_28_11 s_28_10
        let s_28_12: i128 = (s_28_11 + s_28_10);
        // D s_28_13: bit-extract s_28_8 s_28_7 s_28_12
        let s_28_13: Bits = (Bits::new(
            ((s_28_8) >> (s_28_7)).value(),
            u16::try_from(s_28_12).unwrap(),
        ));
        // D s_28_14: cast reint s_28_13 -> u44
        let s_28_14: u64 = (s_28_13.value() as u64);
        // C s_28_15: const #76s : i
        let s_28_15: i128 = 76;
        // D s_28_16: cast zx s_28_3 -> bv
        let s_28_16: Bits = Bits::new(s_28_3 as u128, 128u16);
        // D s_28_17: cast zx s_28_14 -> bv
        let s_28_17: Bits = Bits::new(s_28_14 as u128, 44u16);
        // C s_28_18: const #43s : i
        let s_28_18: i128 = 43;
        // C s_28_19: const #1u : u64
        let s_28_19: u64 = 1;
        // C s_28_20: cast zx s_28_19 -> bv
        let s_28_20: Bits = Bits::new(s_28_19 as u128, 64u16);
        // C s_28_21: lsl s_28_20 s_28_18
        let s_28_21: Bits = s_28_20 << s_28_18;
        // C s_28_22: sub s_28_21 s_28_20
        let s_28_22: Bits = ((s_28_21) - (s_28_20));
        // D s_28_23: and s_28_17 s_28_22
        let s_28_23: Bits = ((s_28_17) & (s_28_22));
        // D s_28_24: lsl s_28_23 s_28_15
        let s_28_24: Bits = s_28_23 << s_28_15;
        // C s_28_25: lsl s_28_22 s_28_15
        let s_28_25: Bits = s_28_22 << s_28_15;
        // C s_28_26: cmpl s_28_25
        let s_28_26: Bits = !s_28_25;
        // D s_28_27: and s_28_16 s_28_26
        let s_28_27: Bits = ((s_28_16) & (s_28_26));
        // D s_28_28: or s_28_27 s_28_24
        let s_28_28: Bits = ((s_28_27) | (s_28_24));
        // D s_28_29: cast reint s_28_28 -> u128
        let s_28_29: u128 = (s_28_28.value() as u128);
        // D s_28_30: call Mk_PAR_EL1_Type(s_28_29)
        let s_28_30: ProductType782ac6922b48c20d = Mk_PAR_EL1_Type(
            state,
            tracer,
            s_28_29,
        );
        // D s_28_31: call PAR_EL1_write(s_28_30)
        let s_28_31: () = PAR_EL1_write(state, tracer, s_28_30);
        // N s_28_32: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call PAR_EL1_read(s_29_0)
        let s_29_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_29_0);
        // D s_29_2: write-var ga#22598 <= s_29_1
        fn_state.ga_22598 = s_29_1;
        // D s_29_3: read-var paspace:u32
        let s_29_3: u32 = fn_state.paspace;
        // C s_29_4: const #1u : u32
        let s_29_4: u32 = 1;
        // D s_29_5: cmp-eq s_29_3 s_29_4
        let s_29_5: bool = ((s_29_3) == (s_29_4));
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
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var ga#22599 <= s_30_0
        fn_state.ga_22599 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var ga#22598:struct
        let s_31_0: ProductType782ac6922b48c20d = fn_state.ga_22598;
        // D s_31_1: read-var ga#22599:u8
        let s_31_1: bool = fn_state.ga_22599;
        // D s_31_2: call _update_PAR_EL1_Type_NS(s_31_0, s_31_1)
        let s_31_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_31_0,
            s_31_1,
        );
        // D s_31_3: call PAR_EL1_write(s_31_2)
        let s_31_3: () = PAR_EL1_write(state, tracer, s_31_2);
        // N s_31_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var ga#22599 <= s_32_0
        fn_state.ga_22599 = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var regime:u32
        let s_33_0: u32 = fn_state.regime;
        // C s_33_1: const #0u : u32
        let s_33_1: u32 = 0;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // N s_33_3: branch s_33_2 b50 b34
        if s_33_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var regime:u32
        let s_34_0: u32 = fn_state.regime;
        // D s_34_1: call SecurityStateForRegime(s_34_0)
        let s_34_1: u32 = SecurityStateForRegime(state, tracer, s_34_0);
        // C s_34_2: const #3u : u32
        let s_34_2: u32 = 3;
        // D s_34_3: cmp-eq s_34_1 s_34_2
        let s_34_3: bool = ((s_34_1) == (s_34_2));
        // N s_34_4: branch s_34_3 b46 b35
        if s_34_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var regime:u32
        let s_35_0: u32 = fn_state.regime;
        // D s_35_1: call SecurityStateForRegime(s_35_0)
        let s_35_1: u32 = SecurityStateForRegime(state, tracer, s_35_0);
        // C s_35_2: const #2u : u32
        let s_35_2: u32 = 2;
        // D s_35_3: cmp-eq s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) == (s_35_2));
        // N s_35_4: branch s_35_3 b37 b36
        if s_35_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call PAR_EL1_read(s_36_0)
        let s_36_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_36_0);
        // C s_36_2: const #() : ()
        let s_36_2: () = ();
        // S s_36_3: call __UNKNOWN_bit(s_36_2)
        let s_36_3: bool = u__UNKNOWN_bit(state, tracer, s_36_2);
        // S s_36_4: call _update_PAR_EL1_Type_NSE(s_36_1, s_36_3)
        let s_36_4: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NSE(
            state,
            tracer,
            s_36_1,
            s_36_3,
        );
        // S s_36_5: call PAR_EL1_write(s_36_4)
        let s_36_5: () = PAR_EL1_write(state, tracer, s_36_4);
        // C s_36_6: const #() : ()
        let s_36_6: () = ();
        // S s_36_7: call PAR_EL1_read(s_36_6)
        let s_36_7: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_36_6);
        // C s_36_8: const #() : ()
        let s_36_8: () = ();
        // S s_36_9: call __UNKNOWN_bit(s_36_8)
        let s_36_9: bool = u__UNKNOWN_bit(state, tracer, s_36_8);
        // S s_36_10: call _update_PAR_EL1_Type_NS(s_36_7, s_36_9)
        let s_36_10: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_36_7,
            s_36_9,
        );
        // S s_36_11: call PAR_EL1_write(s_36_10)
        let s_36_11: () = PAR_EL1_write(state, tracer, s_36_10);
        // N s_36_12: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var regime:u32
        let s_37_0: u32 = fn_state.regime;
        // C s_37_1: const #4u : u32
        let s_37_1: u32 = 4;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // N s_37_3: branch s_37_2 b45 b38
        if s_37_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#29196 <= s_38_0
        fn_state.gs_29196 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#29196:u8
        let s_39_0: bool = fn_state.gs_29196;
        // N s_39_1: branch s_39_0 b44 b40
        if s_39_0 {
            return block_44(state, tracer, fn_state);
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
        // S s_40_1: call PAR_EL1_read(s_40_0)
        let s_40_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_40_0);
        // C s_40_2: const #() : ()
        let s_40_2: () = ();
        // S s_40_3: call __UNKNOWN_bit(s_40_2)
        let s_40_3: bool = u__UNKNOWN_bit(state, tracer, s_40_2);
        // S s_40_4: call _update_PAR_EL1_Type_NSE(s_40_1, s_40_3)
        let s_40_4: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NSE(
            state,
            tracer,
            s_40_1,
            s_40_3,
        );
        // S s_40_5: call PAR_EL1_write(s_40_4)
        let s_40_5: () = PAR_EL1_write(state, tracer, s_40_4);
        // C s_40_6: const #() : ()
        let s_40_6: () = ();
        // S s_40_7: call PAR_EL1_read(s_40_6)
        let s_40_7: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_40_6);
        // D s_40_8: write-var ga#22582 <= s_40_7
        fn_state.ga_22582 = s_40_7;
        // D s_40_9: read-var paspace:u32
        let s_40_9: u32 = fn_state.paspace;
        // C s_40_10: const #3u : u32
        let s_40_10: u32 = 3;
        // D s_40_11: cmp-eq s_40_9 s_40_10
        let s_40_11: bool = ((s_40_9) == (s_40_10));
        // N s_40_12: branch s_40_11 b43 b41
        if s_40_11 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var ga#22583 <= s_41_0
        fn_state.ga_22583 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var ga#22582:struct
        let s_42_0: ProductType782ac6922b48c20d = fn_state.ga_22582;
        // D s_42_1: read-var ga#22583:u8
        let s_42_1: bool = fn_state.ga_22583;
        // D s_42_2: call _update_PAR_EL1_Type_NS(s_42_0, s_42_1)
        let s_42_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_42_0,
            s_42_1,
        );
        // D s_42_3: call PAR_EL1_write(s_42_2)
        let s_42_3: () = PAR_EL1_write(state, tracer, s_42_2);
        // N s_42_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var ga#22583 <= s_43_0
        fn_state.ga_22583 = s_43_0;
        // N s_43_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call PAR_EL1_read(s_44_0)
        let s_44_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_44_0);
        // C s_44_2: const #() : ()
        let s_44_2: () = ();
        // S s_44_3: call __UNKNOWN_bit(s_44_2)
        let s_44_3: bool = u__UNKNOWN_bit(state, tracer, s_44_2);
        // S s_44_4: call _update_PAR_EL1_Type_NSE(s_44_1, s_44_3)
        let s_44_4: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NSE(
            state,
            tracer,
            s_44_1,
            s_44_3,
        );
        // S s_44_5: call PAR_EL1_write(s_44_4)
        let s_44_5: () = PAR_EL1_write(state, tracer, s_44_4);
        // C s_44_6: const #() : ()
        let s_44_6: () = ();
        // S s_44_7: call PAR_EL1_read(s_44_6)
        let s_44_7: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_44_6);
        // C s_44_8: const #() : ()
        let s_44_8: () = ();
        // S s_44_9: call __UNKNOWN_bit(s_44_8)
        let s_44_9: bool = u__UNKNOWN_bit(state, tracer, s_44_8);
        // S s_44_10: call _update_PAR_EL1_Type_NS(s_44_7, s_44_9)
        let s_44_10: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_44_7,
            s_44_9,
        );
        // S s_44_11: call PAR_EL1_write(s_44_10)
        let s_44_11: () = PAR_EL1_write(state, tracer, s_44_10);
        // N s_44_12: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var is_ATS1Ex:u8
        let s_45_0: bool = fn_state.is_ATS1Ex;
        // D s_45_1: write-var gs#29196 <= s_45_0
        fn_state.gs_29196 = s_45_0;
        // N s_45_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call PAR_EL1_read(s_46_0)
        let s_46_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_46_0);
        // C s_46_2: const #() : ()
        let s_46_2: () = ();
        // S s_46_3: call __UNKNOWN_bit(s_46_2)
        let s_46_3: bool = u__UNKNOWN_bit(state, tracer, s_46_2);
        // S s_46_4: call _update_PAR_EL1_Type_NSE(s_46_1, s_46_3)
        let s_46_4: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NSE(
            state,
            tracer,
            s_46_1,
            s_46_3,
        );
        // S s_46_5: call PAR_EL1_write(s_46_4)
        let s_46_5: () = PAR_EL1_write(state, tracer, s_46_4);
        // C s_46_6: const #() : ()
        let s_46_6: () = ();
        // S s_46_7: call PAR_EL1_read(s_46_6)
        let s_46_7: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_46_6);
        // D s_46_8: write-var ga#22565 <= s_46_7
        fn_state.ga_22565 = s_46_7;
        // D s_46_9: read-var paspace:u32
        let s_46_9: u32 = fn_state.paspace;
        // C s_46_10: const #1u : u32
        let s_46_10: u32 = 1;
        // D s_46_11: cmp-eq s_46_9 s_46_10
        let s_46_11: bool = ((s_46_9) == (s_46_10));
        // N s_46_12: branch s_46_11 b49 b47
        if s_46_11 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var ga#22566 <= s_47_0
        fn_state.ga_22566 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var ga#22565:struct
        let s_48_0: ProductType782ac6922b48c20d = fn_state.ga_22565;
        // D s_48_1: read-var ga#22566:u8
        let s_48_1: bool = fn_state.ga_22566;
        // D s_48_2: call _update_PAR_EL1_Type_NS(s_48_0, s_48_1)
        let s_48_2: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_NS(
            state,
            tracer,
            s_48_0,
            s_48_1,
        );
        // D s_48_3: call PAR_EL1_write(s_48_2)
        let s_48_3: () = PAR_EL1_write(state, tracer, s_48_2);
        // N s_48_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var ga#22566 <= s_49_0
        fn_state.ga_22566 = s_49_0;
        // N s_49_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u32
        let s_50_0: u32 = 1;
        // D s_50_1: read-var paspace:u32
        let s_50_1: u32 = fn_state.paspace;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b53 b51
        if s_50_3 {
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
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call PAR_EL1_read(s_51_0)
        let s_51_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_51_0);
        // S s_51_2: call PAR_EL1_write(s_51_1)
        let s_51_2: () = PAR_EL1_write(state, tracer, s_51_1);
        // N s_51_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u32
        let s_53_0: u32 = 0;
        // D s_53_1: read-var paspace:u32
        let s_53_1: u32 = fn_state.paspace;
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // N s_53_4: branch s_53_3 b55 b54
        if s_53_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call PAR_EL1_read(s_54_0)
        let s_54_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_54_0);
        // S s_54_2: call PAR_EL1_write(s_54_1)
        let s_54_2: () = PAR_EL1_write(state, tracer, s_54_1);
        // N s_54_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #2u : u32
        let s_55_0: u32 = 2;
        // D s_55_1: read-var paspace:u32
        let s_55_1: u32 = fn_state.paspace;
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // N s_55_4: branch s_55_3 b57 b56
        if s_55_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #() : ()
        let s_56_0: () = ();
        // S s_56_1: call PAR_EL1_read(s_56_0)
        let s_56_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_56_0);
        // S s_56_2: call PAR_EL1_write(s_56_1)
        let s_56_2: () = PAR_EL1_write(state, tracer, s_56_1);
        // N s_56_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #3u : u32
        let s_57_0: u32 = 3;
        // D s_57_1: read-var paspace:u32
        let s_57_1: u32 = fn_state.paspace;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // N s_57_4: branch s_57_3 b59 b58
        if s_57_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call PAR_EL1_read(s_58_0)
        let s_58_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_58_0);
        // S s_58_2: call PAR_EL1_write(s_58_1)
        let s_58_2: () = PAR_EL1_write(state, tracer, s_58_1);
        // N s_58_3: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call PAR_EL1_read(s_60_0)
        let s_60_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_60_0);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // S s_60_3: call _update_PAR_EL1_Type_D128(s_60_1, s_60_2)
        let s_60_3: ProductType782ac6922b48c20d = u_update_PAR_EL1_Type_D128(
            state,
            tracer,
            s_60_1,
            s_60_2,
        );
        // S s_60_4: call PAR_EL1_write(s_60_3)
        let s_60_4: () = PAR_EL1_write(state, tracer, s_60_3);
        // N s_60_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
