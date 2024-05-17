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
use u__UNKNOWN_bits::*;
use AArch64_AbortSyndrome::*;
use u_get_MDCR_EL2_Type_TDE::*;
use EL2Enabled::*;
use AArch64_TakeException::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_VectorCatchException<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_9860: bool,
        gs_9861: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
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
        // C s_0_3: const #432u : u32
        let s_0_3: u32 = 432;
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
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call EL2Enabled(s_0_8)
        let s_0_9: bool = EL2Enabled(state, tracer, s_0_8);
        // N s_0_10: branch s_0_9 b3 b1
        if s_0_9 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#9861 <= s_1_0
        fn_state.gs_9861 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#9861:u8
        let s_2_0: bool = fn_state.gs_9861;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #64s : i64
        let s_2_2: i64 = 64;
        // C s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // S s_2_4: call ThisInstrAddr(s_2_3)
        let s_2_4: Bits = ThisInstrAddr(state, tracer, s_2_3);
        // S s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // C s_2_6: const #0u : u8
        let s_2_6: u8 = 0;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // C s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // C s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #64s : i64
        let s_2_10: i64 = 64;
        // C s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // S s_2_12: call __UNKNOWN_bits(s_2_11)
        let s_2_12: Bits = u__UNKNOWN_bits(state, tracer, s_2_11);
        // S s_2_13: cast reint s_2_12 -> u64
        let s_2_13: u64 = (s_2_12.value() as u64);
        // C s_2_14: const #30u : u32
        let s_2_14: u32 = 30;
        // D s_2_15: read-var fault:struct
        let s_2_15: ProductType1d757adad216cdef = fn_state.fault;
        // C s_2_16: const #432u : u32
        let s_2_16: u32 = 432;
        // D s_2_17: read-reg s_2_16:u8
        let s_2_17: u8 = {
            let value = state.read_register::<u8>(s_2_16 as isize);
            tracer.read_register(s_2_16 as isize, value);
            value
        };
        // D s_2_18: call AArch64_AbortSyndrome(s_2_14, s_2_15, s_2_13, s_2_17)
        let s_2_18: ProductTypeb7f99f96751e17c4 = AArch64_AbortSyndrome(
            state,
            tracer,
            s_2_14,
            s_2_15,
            s_2_13,
            s_2_17,
        );
        // C s_2_19: cast zx s_2_9 -> i
        let s_2_19: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_20: const #432u : u32
        let s_2_20: u32 = 432;
        // D s_2_21: read-reg s_2_20:u8
        let s_2_21: u8 = {
            let value = state.read_register::<u8>(s_2_20 as isize);
            tracer.read_register(s_2_20 as isize, value);
            value
        };
        // D s_2_22: call AArch64_TakeException(s_2_21, s_2_18, s_2_5, s_2_19)
        let s_2_22: () = AArch64_TakeException(
            state,
            tracer,
            s_2_21,
            s_2_18,
            s_2_5,
            s_2_19,
        );
        // N s_2_23: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #102552u : u32
        let s_3_0: u32 = 102552;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_HCR_EL2_Type_TGE(s_3_1)
        let s_3_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // C s_3_4: const #1u : u8
        let s_3_4: bool = true;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // D s_3_6: cmp-eq s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) == (s_3_5));
        // N s_3_7: branch s_3_6 b6 b4
        if s_3_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #104880u : u32
        let s_4_0: u32 = 104880;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_MDCR_EL2_Type_TDE(s_4_1)
        let s_4_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var gs#9860 <= s_4_6
        fn_state.gs_9860 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#9860:u8
        let s_5_0: bool = fn_state.gs_9860;
        // D s_5_1: write-var gs#9861 <= s_5_0
        fn_state.gs_9861 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var gs#9860 <= s_6_0
        fn_state.gs_9860 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
