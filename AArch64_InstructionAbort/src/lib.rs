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
use ThisInstrAddr::*;
use AArch64_SyncExternalAbortTarget::*;
use take_exception::*;
use IsSecondStage::*;
use AArch64_TakeException::*;
use HaveRME::*;
use HaveDoubleFaultExt::*;
use AArch64_AbortSyndrome::*;
use AArch64_RouteToSErrorOffset::*;
use EL2Enabled::*;
use IsExternalAbort__1::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_GPF::*;
use common::*;
pub fn AArch64_InstructionAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9853: bool,
        gs_9855: bool,
        preferred_exception_return: u64,
        gs_9847: bool,
        gs_9848: bool,
        route_to_el2shadow_130: bool,
        vect_offset: i128,
        ga_7090: ProductType396b95aa74979079,
        gs_9852: bool,
        gs_9849: bool,
        gs_9846: bool,
        gs_9850: bool,
        gs_9851: bool,
        target_el: u8,
        vaddress: u64,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        vaddress,
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveDoubleFaultExt(s_0_0)
        let s_0_1: bool = HaveDoubleFaultExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b40 b1
        if s_0_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var fault:struct
        let s_2_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_2_1: call IsExternalAbort__1(s_2_0)
        let s_2_1: bool = IsExternalAbort__1(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b39 b3
        if s_2_1 {
            return block_39(state, tracer, fn_state);
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
        // S s_3_1: call EL2Enabled(s_3_0)
        let s_3_1: bool = EL2Enabled(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b35 b4
        if s_3_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#9847 <= s_4_0
        fn_state.gs_9847 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#9847:u8
        let s_5_0: bool = fn_state.gs_9847;
        // N s_5_1: branch s_5_0 b22 b6
        if s_5_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#9852 <= s_6_0
        fn_state.gs_9852 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#9852:u8
        let s_7_0: bool = fn_state.gs_9852;
        // D s_7_1: write-var route_to_el2shadow#130 <= s_7_0
        fn_state.route_to_el2shadow_130 = s_7_0;
        // C s_7_2: const #16975u : u32
        let s_7_2: u32 = 16975;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // C s_7_5: const #424u : u32
        let s_7_5: u32 = 424;
        // D s_7_6: read-reg s_7_5:u8
        let s_7_6: u8 = {
            let value = state.read_register::<u8>(s_7_5 as isize);
            tracer.read_register(s_7_5 as isize, value);
            value
        };
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 2u16);
        // D s_7_8: cmp-eq s_7_4 s_7_7
        let s_7_8: bool = ((s_7_4) == (s_7_7));
        // N s_7_9: branch s_7_8 b21 b8
        if s_7_8 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #432u : u32
        let s_8_3: u32 = 432;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // N s_8_7: branch s_8_6 b20 b9
        if s_8_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var route_to_el2shadow#130:u8
        let s_9_0: bool = fn_state.route_to_el2shadow_130;
        // D s_9_1: write-var gs#9853 <= s_9_0
        fn_state.gs_9853 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#9853:u8
        let s_10_0: bool = fn_state.gs_9853;
        // N s_10_1: branch s_10_0 b19 b11
        if s_10_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #440u : u32
        let s_11_0: u32 = 440;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: write-var target_el <= s_11_1
        fn_state.target_el = s_11_1;
        // N s_11_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // C s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // S s_12_2: call ThisInstrAddr(s_12_1)
        let s_12_2: Bits = ThisInstrAddr(state, tracer, s_12_1);
        // S s_12_3: cast reint s_12_2 -> u64
        let s_12_3: u64 = (s_12_2.value() as u64);
        // D s_12_4: write-var preferred_exception_return <= s_12_3
        fn_state.preferred_exception_return = s_12_3;
        // D s_12_5: read-var fault:struct
        let s_12_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_12_6: call IsExternalAbort__1(s_12_5)
        let s_12_6: bool = IsExternalAbort__1(state, tracer, s_12_5);
        // N s_12_7: branch s_12_6 b18 b13
        if s_12_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#9855 <= s_13_0
        fn_state.gs_9855 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#9855:u8
        let s_14_0: bool = fn_state.gs_9855;
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
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: u8 = 0;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 4u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // D s_15_3: write-var vect_offset <= s_15_2
        fn_state.vect_offset = s_15_2;
        // N s_15_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var vect_offset:i
        let s_16_0: i128 = fn_state.vect_offset;
        // C s_16_1: const #17u : u32
        let s_16_1: u32 = 17;
        // D s_16_2: read-var fault:struct
        let s_16_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_3: read-var vaddress:u64
        let s_16_3: u64 = fn_state.vaddress;
        // D s_16_4: read-var target_el:u8
        let s_16_4: u8 = fn_state.target_el;
        // D s_16_5: call AArch64_AbortSyndrome(s_16_1, s_16_2, s_16_3, s_16_4)
        let s_16_5: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_16_1,
            s_16_2,
            s_16_3,
            s_16_4,
        );
        // D s_16_6: read-var fault:struct
        let s_16_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_16_7: create-sum enum = 1:"s_16_6"
        let s_16_7: SumTypea0f5ebb1a394e20b = SumTypea0f5ebb1a394e20b::_1(s_16_6);
        // D s_16_8: call take_exception(s_16_7)
        let s_16_8: () = take_exception(state, tracer, s_16_7);
        // D s_16_9: read-var target_el:u8
        let s_16_9: u8 = fn_state.target_el;
        // D s_16_10: read-var preferred_exception_return:u64
        let s_16_10: u64 = fn_state.preferred_exception_return;
        // D s_16_11: call AArch64_TakeException(s_16_9, s_16_5, s_16_10, s_16_0)
        let s_16_11: () = AArch64_TakeException(
            state,
            tracer,
            s_16_9,
            s_16_5,
            s_16_10,
            s_16_0,
        );
        // N s_16_12: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #384u : u12
        let s_17_0: u16 = 384;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 12u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // D s_17_3: write-var vect_offset <= s_17_2
        fn_state.vect_offset = s_17_2;
        // N s_17_4: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var target_el:u8
        let s_18_0: u8 = fn_state.target_el;
        // D s_18_1: call AArch64_RouteToSErrorOffset(s_18_0)
        let s_18_1: bool = AArch64_RouteToSErrorOffset(state, tracer, s_18_0);
        // D s_18_2: write-var gs#9855 <= s_18_1
        fn_state.gs_9855 = s_18_1;
        // N s_18_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #432u : u32
        let s_19_0: u32 = 432;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: u8 = {
            let value = state.read_register::<u8>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: write-var target_el <= s_19_1
        fn_state.target_el = s_19_1;
        // N s_19_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var gs#9853 <= s_20_0
        fn_state.gs_9853 = s_20_0;
        // N s_20_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #424u : u32
        let s_21_0: u32 = 424;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: u8 = {
            let value = state.read_register::<u8>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: write-var target_el <= s_21_1
        fn_state.target_el = s_21_1;
        // N s_21_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // N s_22_7: branch s_22_6 b34 b23
        if s_22_6 {
            return block_34(state, tracer, fn_state);
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
        // S s_23_1: call HaveRME(s_23_0)
        let s_23_1: bool = HaveRME(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b33 b24
        if s_23_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#9848 <= s_24_0
        fn_state.gs_9848 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#9848:u8
        let s_25_0: bool = fn_state.gs_9848;
        // N s_25_1: branch s_25_0 b32 b26
        if s_25_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#9849 <= s_26_0
        fn_state.gs_9849 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#9849:u8
        let s_27_0: bool = fn_state.gs_9849;
        // D s_27_1: write-var gs#9850 <= s_27_0
        fn_state.gs_9850 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#9850:u8
        let s_28_0: bool = fn_state.gs_9850;
        // N s_28_1: branch s_28_0 b31 b29
        if s_28_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var fault:struct
        let s_29_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_29_1: call IsSecondStage(s_29_0)
        let s_29_1: bool = IsSecondStage(state, tracer, s_29_0);
        // D s_29_2: write-var gs#9851 <= s_29_1
        fn_state.gs_9851 = s_29_1;
        // N s_29_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#9851:u8
        let s_30_0: bool = fn_state.gs_9851;
        // D s_30_1: write-var gs#9852 <= s_30_0
        fn_state.gs_9852 = s_30_0;
        // N s_30_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#9851 <= s_31_0
        fn_state.gs_9851 = s_31_0;
        // N s_31_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #102552u : u32
        let s_32_0: u32 = 102552;
        // D s_32_1: read-reg s_32_0:struct
        let s_32_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call _get_HCR_EL2_Type_GPF(s_32_1)
        let s_32_2: bool = u_get_HCR_EL2_Type_GPF(state, tracer, s_32_1);
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // C s_32_4: const #1u : u8
        let s_32_4: bool = true;
        // C s_32_5: cast zx s_32_4 -> bv
        let s_32_5: Bits = Bits::new(s_32_4 as u128, 1u16);
        // D s_32_6: cmp-eq s_32_3 s_32_5
        let s_32_6: bool = ((s_32_3) == (s_32_5));
        // D s_32_7: write-var gs#9849 <= s_32_6
        fn_state.gs_9849 = s_32_6;
        // N s_32_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var fault.6:struct
        let s_33_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_33_1: write-var ga#7090 <= s_33_0
        fn_state.ga_7090 = s_33_0;
        // D s_33_2: read-var ga#7090.0:struct
        let s_33_2: u32 = fn_state.ga_7090._0;
        // C s_33_3: const #4u : u32
        let s_33_3: u32 = 4;
        // D s_33_4: cmp-eq s_33_2 s_33_3
        let s_33_4: bool = ((s_33_2) == (s_33_3));
        // D s_33_5: write-var gs#9848 <= s_33_4
        fn_state.gs_9848 = s_33_4;
        // N s_33_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#9850 <= s_34_0
        fn_state.gs_9850 = s_34_0;
        // N s_34_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #16975u : u32
        let s_35_0: u32 = 16975;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 2u16);
        // C s_35_3: const #448u : u32
        let s_35_3: u32 = 448;
        // D s_35_4: read-reg s_35_3:u8
        let s_35_4: u8 = {
            let value = state.read_register::<u8>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 2u16);
        // D s_35_6: cmp-eq s_35_2 s_35_5
        let s_35_6: bool = ((s_35_2) == (s_35_5));
        // N s_35_7: branch s_35_6 b38 b36
        if s_35_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
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
        // C s_36_3: const #440u : u32
        let s_36_3: u32 = 440;
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
        // D s_36_7: write-var gs#9846 <= s_36_6
        fn_state.gs_9846 = s_36_6;
        // N s_36_8: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#9846:u8
        let s_37_0: bool = fn_state.gs_9846;
        // D s_37_1: write-var gs#9847 <= s_37_0
        fn_state.gs_9847 = s_37_0;
        // N s_37_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#9846 <= s_38_0
        fn_state.gs_9846 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var fault:struct
        let s_39_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_39_1: call AArch64_SyncExternalAbortTarget(s_39_0)
        let s_39_1: u8 = AArch64_SyncExternalAbortTarget(state, tracer, s_39_0);
        // D s_39_2: write-var target_el <= s_39_1
        fn_state.target_el = s_39_1;
        // N s_39_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var fault.16:struct
        let s_40_0: u32 = fn_state.fault._16;
        // C s_40_1: const #15u : u32
        let s_40_1: u32 = 15;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // N s_40_3: assert s_40_2
        let s_40_3: () = assert!(s_40_2);
        // N s_40_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
