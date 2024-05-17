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
use AArch64_AbortSyndrome::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_EL2_Type_TGE::*;
use AArch64_TakeException::*;
use HaveNV2Ext::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_WatchpointException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    vaddress: u64,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        gs_9872: bool,
        gs_9871: bool,
        vect_offset: i64,
        except: ProductTypeb7f99f96751e17c4,
        gs_9876: bool,
        preferred_exception_return: u64,
        ga_7115: ProductType9878976b5bcce9c9,
        gs_9875: bool,
        target_el: u8,
        gs_9870: bool,
        gs_9869: bool,
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
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #448u : u32
        let s_0_11: u32 = 448;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) == (s_0_13));
        // N s_0_15: branch s_0_14 b24 b1
        if s_0_14 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#9869 <= s_1_6
        fn_state.gs_9869 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9869:u8
        let s_2_0: bool = fn_state.gs_9869;
        // N s_2_1: branch s_2_0 b23 b3
        if s_2_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#9870 <= s_3_0
        fn_state.gs_9870 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#9870:u8
        let s_4_0: bool = fn_state.gs_9870;
        // N s_4_1: branch s_4_0 b19 b5
        if s_4_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#9872 <= s_5_0
        fn_state.gs_9872 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#9872:u8
        let s_6_0: bool = fn_state.gs_9872;
        // D s_6_1: write-var route_to_el2 <= s_6_0
        fn_state.route_to_el2 = s_6_0;
        // C s_6_2: const #64s : i64
        let s_6_2: i64 = 64;
        // C s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_4: call ThisInstrAddr(s_6_3)
        let s_6_4: Bits = ThisInstrAddr(state, tracer, s_6_3);
        // S s_6_5: cast reint s_6_4 -> u64
        let s_6_5: u64 = (s_6_4.value() as u64);
        // D s_6_6: write-var preferred_exception_return <= s_6_5
        fn_state.preferred_exception_return = s_6_5;
        // C s_6_7: const #0u : u8
        let s_6_7: u8 = 0;
        // C s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 4u16);
        // C s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (s_6_8.value() as i128);
        // C s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: write-var vect_offset <= s_6_10
        fn_state.vect_offset = s_6_10;
        // C s_6_12: const #16975u : u32
        let s_6_12: u32 = 16975;
        // D s_6_13: read-reg s_6_12:u8
        let s_6_13: u8 = {
            let value = state.read_register::<u8>(s_6_12 as isize);
            tracer.read_register(s_6_12 as isize, value);
            value
        };
        // D s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 2u16);
        // C s_6_15: const #432u : u32
        let s_6_15: u32 = 432;
        // D s_6_16: read-reg s_6_15:u8
        let s_6_16: u8 = {
            let value = state.read_register::<u8>(s_6_15 as isize);
            tracer.read_register(s_6_15 as isize, value);
            value
        };
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 2u16);
        // D s_6_18: cmp-eq s_6_14 s_6_17
        let s_6_18: bool = ((s_6_14) == (s_6_17));
        // N s_6_19: branch s_6_18 b18 b7
        if s_6_18 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var route_to_el2:u8
        let s_7_0: bool = fn_state.route_to_el2;
        // D s_7_1: write-var gs#9875 <= s_7_0
        fn_state.gs_9875 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#9875:u8
        let s_8_0: bool = fn_state.gs_9875;
        // N s_8_1: branch s_8_0 b17 b9
        if s_8_0 {
            return block_17(state, tracer, fn_state);
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
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveNV2Ext(s_10_0)
        let s_10_1: bool = HaveNV2Ext(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b16 b11
        if s_10_1 {
            return block_16(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#9876 <= s_11_0
        fn_state.gs_9876 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#9876:u8
        let s_12_0: bool = fn_state.gs_9876;
        // N s_12_1: branch s_12_0 b15 b13
        if s_12_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #27u : u32
        let s_13_0: u32 = 27;
        // D s_13_1: read-var fault:struct
        let s_13_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_13_2: read-var vaddress:u64
        let s_13_2: u64 = fn_state.vaddress;
        // D s_13_3: read-var target_el:u8
        let s_13_3: u8 = fn_state.target_el;
        // D s_13_4: call AArch64_AbortSyndrome(s_13_0, s_13_1, s_13_2, s_13_3)
        let s_13_4: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
            s_13_3,
        );
        // D s_13_5: write-var except <= s_13_4
        fn_state.except = s_13_4;
        // N s_13_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var vect_offset:i64
        let s_14_0: i64 = fn_state.vect_offset;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var target_el:u8
        let s_14_2: u8 = fn_state.target_el;
        // D s_14_3: read-var except:struct
        let s_14_3: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_14_4: read-var preferred_exception_return:u64
        let s_14_4: u64 = fn_state.preferred_exception_return;
        // D s_14_5: call AArch64_TakeException(s_14_2, s_14_3, s_14_4, s_14_1)
        let s_14_5: () = AArch64_TakeException(
            state,
            tracer,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_1,
        );
        // N s_14_6: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #28u : u32
        let s_15_0: u32 = 28;
        // D s_15_1: read-var fault:struct
        let s_15_1: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_2: read-var vaddress:u64
        let s_15_2: u64 = fn_state.vaddress;
        // D s_15_3: read-var target_el:u8
        let s_15_3: u8 = fn_state.target_el;
        // D s_15_4: call AArch64_AbortSyndrome(s_15_0, s_15_1, s_15_2, s_15_3)
        let s_15_4: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
            s_15_3,
        );
        // D s_15_5: write-var except <= s_15_4
        fn_state.except = s_15_4;
        // N s_15_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var fault.0:struct
        let s_16_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_16_1: write-var ga#7115 <= s_16_0
        fn_state.ga_7115 = s_16_0;
        // D s_16_2: read-var ga#7115.1:struct
        let s_16_2: u32 = fn_state.ga_7115._1;
        // C s_16_3: const #9u : u32
        let s_16_3: u32 = 9;
        // D s_16_4: cmp-eq s_16_2 s_16_3
        let s_16_4: bool = ((s_16_2) == (s_16_3));
        // D s_16_5: write-var gs#9876 <= s_16_4
        fn_state.gs_9876 = s_16_4;
        // N s_16_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #432u : u32
        let s_17_0: u32 = 432;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: write-var target_el <= s_17_1
        fn_state.target_el = s_17_1;
        // N s_17_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#9875 <= s_18_0
        fn_state.gs_9875 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #102552u : u32
        let s_19_0: u32 = 102552;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_HCR_EL2_Type_TGE(s_19_1)
        let s_19_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #1u : u8
        let s_19_4: bool = true;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // N s_19_7: branch s_19_6 b22 b20
        if s_19_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #104880u : u32
        let s_20_0: u32 = 104880;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_MDCR_EL2_Type_TDE(s_20_1)
        let s_20_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #1u : u8
        let s_20_4: bool = true;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // D s_20_7: write-var gs#9871 <= s_20_6
        fn_state.gs_9871 = s_20_6;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#9871:u8
        let s_21_0: bool = fn_state.gs_9871;
        // D s_21_1: write-var gs#9872 <= s_21_0
        fn_state.gs_9872 = s_21_0;
        // N s_21_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#9871 <= s_22_0
        fn_state.gs_9871 = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL2Enabled(s_23_0)
        let s_23_1: bool = EL2Enabled(state, tracer, s_23_0);
        // D s_23_2: write-var gs#9870 <= s_23_1
        fn_state.gs_9870 = s_23_1;
        // N s_23_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#9869 <= s_24_0
        fn_state.gs_9869 = s_24_0;
        // N s_24_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
