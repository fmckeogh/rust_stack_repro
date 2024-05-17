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
use u_get_HCR_EL2_Type_GPF::*;
use AArch64_SyncExternalAbortTarget::*;
use take_exception::*;
use IsSecondStage::*;
use AArch64_TakeException::*;
use HaveRME::*;
use AArch64_RouteToSErrorOffset::*;
use AArch64_AbortSyndrome::*;
use EL2Enabled::*;
use IsExternalAbort__1::*;
use HaveNV2Ext::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_DataAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9827: bool,
        gs_9840: bool,
        gs_9832: bool,
        ga_7062: ProductType9878976b5bcce9c9,
        except: ProductTypeb7f99f96751e17c4,
        gs_9835: bool,
        preferred_exception_return: u64,
        gs_9826: bool,
        route_to_el2shadow_128: bool,
        gs_9829: bool,
        vect_offset: i128,
        vect_offsetshadow_127: i128,
        gs_9837: bool,
        gs_9830: bool,
        gs_9833: bool,
        gs_9828: bool,
        gs_9834: bool,
        gs_9831: bool,
        ga_7056: ProductType396b95aa74979079,
        ga_7070: ProductType9878976b5bcce9c9,
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
        // D s_0_0: read-var fault:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_1: call IsExternalAbort__1(s_0_0)
        let s_0_1: bool = IsExternalAbort__1(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b49 b1
        if s_0_1 {
            return block_49(state, tracer, fn_state);
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
        // S s_1_1: call EL2Enabled(s_1_0)
        let s_1_1: bool = EL2Enabled(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b45 b2
        if s_1_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#9827 <= s_2_0
        fn_state.gs_9827 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#9827:u8
        let s_3_0: bool = fn_state.gs_9827;
        // N s_3_1: branch s_3_0 b26 b4
        if s_3_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#9834 <= s_4_0
        fn_state.gs_9834 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#9834:u8
        let s_5_0: bool = fn_state.gs_9834;
        // D s_5_1: write-var route_to_el2shadow#128 <= s_5_0
        fn_state.route_to_el2shadow_128 = s_5_0;
        // C s_5_2: const #16975u : u32
        let s_5_2: u32 = 16975;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // C s_5_5: const #424u : u32
        let s_5_5: u32 = 424;
        // D s_5_6: read-reg s_5_5:u8
        let s_5_6: u8 = {
            let value = state.read_register::<u8>(s_5_5 as isize);
            tracer.read_register(s_5_5 as isize, value);
            value
        };
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 2u16);
        // D s_5_8: cmp-eq s_5_4 s_5_7
        let s_5_8: bool = ((s_5_4) == (s_5_7));
        // N s_5_9: branch s_5_8 b25 b6
        if s_5_8 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #432u : u32
        let s_6_3: u32 = 432;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b24 b7
        if s_6_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var route_to_el2shadow#128:u8
        let s_7_0: bool = fn_state.route_to_el2shadow_128;
        // D s_7_1: write-var gs#9835 <= s_7_0
        fn_state.gs_9835 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9835:u8
        let s_8_0: bool = fn_state.gs_9835;
        // N s_8_1: branch s_8_0 b23 b9
        if s_8_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: write-var target_el <= s_9_1
        fn_state.target_el = s_9_1;
        // N s_9_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // S s_10_2: call ThisInstrAddr(s_10_1)
        let s_10_2: Bits = ThisInstrAddr(state, tracer, s_10_1);
        // S s_10_3: cast reint s_10_2 -> u64
        let s_10_3: u64 = (s_10_2.value() as u64);
        // D s_10_4: write-var preferred_exception_return <= s_10_3
        fn_state.preferred_exception_return = s_10_3;
        // D s_10_5: read-var fault:struct
        let s_10_5: ProductType1d757adad216cdef = fn_state.fault;
        // D s_10_6: call IsExternalAbort__1(s_10_5)
        let s_10_6: bool = IsExternalAbort__1(state, tracer, s_10_5);
        // N s_10_7: branch s_10_6 b22 b11
        if s_10_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#9837 <= s_11_0
        fn_state.gs_9837 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#9837:u8
        let s_12_0: bool = fn_state.gs_9837;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
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
        let s_13_0: u8 = 0;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // D s_13_3: write-var vect_offset <= s_13_2
        fn_state.vect_offset = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var vect_offset:i
        let s_14_0: i128 = fn_state.vect_offset;
        // D s_14_1: write-var vect_offsetshadow#127 <= s_14_0
        fn_state.vect_offsetshadow_127 = s_14_0;
        // C s_14_2: const #() : ()
        let s_14_2: () = ();
        // S s_14_3: call HaveNV2Ext(s_14_2)
        let s_14_3: bool = HaveNV2Ext(state, tracer, s_14_2);
        // N s_14_4: branch s_14_3 b20 b15
        if s_14_3 {
            return block_20(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#9840 <= s_15_0
        fn_state.gs_9840 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#9840:u8
        let s_16_0: bool = fn_state.gs_9840;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #19u : u32
        let s_17_0: u32 = 19;
        // D s_17_1: read-var fault:struct
        let s_17_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_17_2: read-var vaddress:u64
        let s_17_2: u64 = fn_state.vaddress;
        // D s_17_3: read-var target_el:u8
        let s_17_3: u8 = fn_state.target_el;
        // D s_17_4: call AArch64_AbortSyndrome(s_17_0, s_17_1, s_17_2, s_17_3)
        let s_17_4: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_17_0,
            s_17_1,
            s_17_2,
            s_17_3,
        );
        // D s_17_5: write-var except <= s_17_4
        fn_state.except = s_17_4;
        // N s_17_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var fault:struct
        let s_18_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_18_1: create-sum enum = 1:"s_18_0"
        let s_18_1: SumTypea0f5ebb1a394e20b = SumTypea0f5ebb1a394e20b::_1(s_18_0);
        // D s_18_2: call take_exception(s_18_1)
        let s_18_2: () = take_exception(state, tracer, s_18_1);
        // D s_18_3: read-var target_el:u8
        let s_18_3: u8 = fn_state.target_el;
        // D s_18_4: read-var except:struct
        let s_18_4: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_18_5: read-var preferred_exception_return:u64
        let s_18_5: u64 = fn_state.preferred_exception_return;
        // D s_18_6: read-var vect_offsetshadow#127:i
        let s_18_6: i128 = fn_state.vect_offsetshadow_127;
        // D s_18_7: call AArch64_TakeException(s_18_3, s_18_4, s_18_5, s_18_6)
        let s_18_7: () = AArch64_TakeException(
            state,
            tracer,
            s_18_3,
            s_18_4,
            s_18_5,
            s_18_6,
        );
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #20u : u32
        let s_19_0: u32 = 20;
        // D s_19_1: read-var fault:struct
        let s_19_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_19_2: read-var vaddress:u64
        let s_19_2: u64 = fn_state.vaddress;
        // D s_19_3: read-var target_el:u8
        let s_19_3: u8 = fn_state.target_el;
        // D s_19_4: call AArch64_AbortSyndrome(s_19_0, s_19_1, s_19_2, s_19_3)
        let s_19_4: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_19_0,
            s_19_1,
            s_19_2,
            s_19_3,
        );
        // D s_19_5: write-var except <= s_19_4
        fn_state.except = s_19_4;
        // N s_19_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var fault.0:struct
        let s_20_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_20_1: write-var ga#7070 <= s_20_0
        fn_state.ga_7070 = s_20_0;
        // D s_20_2: read-var ga#7070.1:struct
        let s_20_2: u32 = fn_state.ga_7070._1;
        // C s_20_3: const #9u : u32
        let s_20_3: u32 = 9;
        // D s_20_4: cmp-eq s_20_2 s_20_3
        let s_20_4: bool = ((s_20_2) == (s_20_3));
        // D s_20_5: write-var gs#9840 <= s_20_4
        fn_state.gs_9840 = s_20_4;
        // N s_20_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #384u : u12
        let s_21_0: u16 = 384;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 12u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // D s_21_3: write-var vect_offset <= s_21_2
        fn_state.vect_offset = s_21_2;
        // N s_21_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var target_el:u8
        let s_22_0: u8 = fn_state.target_el;
        // D s_22_1: call AArch64_RouteToSErrorOffset(s_22_0)
        let s_22_1: bool = AArch64_RouteToSErrorOffset(state, tracer, s_22_0);
        // D s_22_2: write-var gs#9837 <= s_22_1
        fn_state.gs_9837 = s_22_1;
        // N s_22_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #432u : u32
        let s_23_0: u32 = 432;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: write-var target_el <= s_23_1
        fn_state.target_el = s_23_1;
        // N s_23_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#9835 <= s_24_0
        fn_state.gs_9835 = s_24_0;
        // N s_24_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #424u : u32
        let s_25_0: u32 = 424;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: write-var target_el <= s_25_1
        fn_state.target_el = s_25_1;
        // N s_25_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #102552u : u32
        let s_26_0: u32 = 102552;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_HCR_EL2_Type_TGE(s_26_1)
        let s_26_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_26_1);
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // C s_26_4: const #1u : u8
        let s_26_4: bool = true;
        // C s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 1u16);
        // D s_26_6: cmp-eq s_26_3 s_26_5
        let s_26_6: bool = ((s_26_3) == (s_26_5));
        // N s_26_7: branch s_26_6 b44 b27
        if s_26_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call HaveRME(s_27_0)
        let s_27_1: bool = HaveRME(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b43 b28
        if s_27_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#9828 <= s_28_0
        fn_state.gs_9828 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#9828:u8
        let s_29_0: bool = fn_state.gs_9828;
        // N s_29_1: branch s_29_0 b42 b30
        if s_29_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#9829 <= s_30_0
        fn_state.gs_9829 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#9829:u8
        let s_31_0: bool = fn_state.gs_9829;
        // D s_31_1: write-var gs#9830 <= s_31_0
        fn_state.gs_9830 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#9830:u8
        let s_32_0: bool = fn_state.gs_9830;
        // N s_32_1: branch s_32_0 b41 b33
        if s_32_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call HaveNV2Ext(s_33_0)
        let s_33_1: bool = HaveNV2Ext(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b40 b34
        if s_33_1 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#9831 <= s_34_0
        fn_state.gs_9831 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#9831:u8
        let s_35_0: bool = fn_state.gs_9831;
        // D s_35_1: write-var gs#9832 <= s_35_0
        fn_state.gs_9832 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#9832:u8
        let s_36_0: bool = fn_state.gs_9832;
        // N s_36_1: branch s_36_0 b39 b37
        if s_36_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var fault:struct
        let s_37_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_37_1: call IsSecondStage(s_37_0)
        let s_37_1: bool = IsSecondStage(state, tracer, s_37_0);
        // D s_37_2: write-var gs#9833 <= s_37_1
        fn_state.gs_9833 = s_37_1;
        // N s_37_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#9833:u8
        let s_38_0: bool = fn_state.gs_9833;
        // D s_38_1: write-var gs#9834 <= s_38_0
        fn_state.gs_9834 = s_38_0;
        // N s_38_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#9833 <= s_39_0
        fn_state.gs_9833 = s_39_0;
        // N s_39_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var fault.0:struct
        let s_40_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_40_1: write-var ga#7062 <= s_40_0
        fn_state.ga_7062 = s_40_0;
        // D s_40_2: read-var ga#7062.1:struct
        let s_40_2: u32 = fn_state.ga_7062._1;
        // C s_40_3: const #9u : u32
        let s_40_3: u32 = 9;
        // D s_40_4: cmp-eq s_40_2 s_40_3
        let s_40_4: bool = ((s_40_2) == (s_40_3));
        // D s_40_5: write-var gs#9831 <= s_40_4
        fn_state.gs_9831 = s_40_4;
        // N s_40_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#9832 <= s_41_0
        fn_state.gs_9832 = s_41_0;
        // N s_41_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_GPF(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_GPF(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #1u : u8
        let s_42_4: bool = true;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#9829 <= s_42_6
        fn_state.gs_9829 = s_42_6;
        // N s_42_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var fault.6:struct
        let s_43_0: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_43_1: write-var ga#7056 <= s_43_0
        fn_state.ga_7056 = s_43_0;
        // D s_43_2: read-var ga#7056.0:struct
        let s_43_2: u32 = fn_state.ga_7056._0;
        // C s_43_3: const #4u : u32
        let s_43_3: u32 = 4;
        // D s_43_4: cmp-eq s_43_2 s_43_3
        let s_43_4: bool = ((s_43_2) == (s_43_3));
        // D s_43_5: write-var gs#9828 <= s_43_4
        fn_state.gs_9828 = s_43_4;
        // N s_43_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#9830 <= s_44_0
        fn_state.gs_9830 = s_44_0;
        // N s_44_2: jump b32
        return block_32(state, tracer, fn_state);
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
        // C s_45_3: const #448u : u32
        let s_45_3: u32 = 448;
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
        // N s_45_7: branch s_45_6 b48 b46
        if s_45_6 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #16975u : u32
        let s_46_0: u32 = 16975;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: cast zx s_46_1 -> bv
        let s_46_2: Bits = Bits::new(s_46_1 as u128, 2u16);
        // C s_46_3: const #440u : u32
        let s_46_3: u32 = 440;
        // D s_46_4: read-reg s_46_3:u8
        let s_46_4: u8 = {
            let value = state.read_register::<u8>(s_46_3 as isize);
            tracer.read_register(s_46_3 as isize, value);
            value
        };
        // D s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 2u16);
        // D s_46_6: cmp-eq s_46_2 s_46_5
        let s_46_6: bool = ((s_46_2) == (s_46_5));
        // D s_46_7: write-var gs#9826 <= s_46_6
        fn_state.gs_9826 = s_46_6;
        // N s_46_8: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#9826:u8
        let s_47_0: bool = fn_state.gs_9826;
        // D s_47_1: write-var gs#9827 <= s_47_0
        fn_state.gs_9827 = s_47_0;
        // N s_47_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#9826 <= s_48_0
        fn_state.gs_9826 = s_48_0;
        // N s_48_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var fault:struct
        let s_49_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_49_1: call AArch64_SyncExternalAbortTarget(s_49_0)
        let s_49_1: u8 = AArch64_SyncExternalAbortTarget(state, tracer, s_49_0);
        // D s_49_2: write-var target_el <= s_49_1
        fn_state.target_el = s_49_1;
        // N s_49_3: jump b10
        return block_10(state, tracer, fn_state);
    }
}
