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
use Mk_SPSR_EL3_Type::*;
use u__UNKNOWN_bits::*;
use Mk_SPSR_EL2_Type::*;
use HaveAArch32EL::*;
use Mk_DSPSR_EL0_Type::*;
use Mk_SPSR_EL1_Type::*;
use common::*;
pub fn AArch64_ResetSpecialRegisters<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_15560: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_15560: (),
    }
    let fn_state = FunctionState {
        gs_15560,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call __UNKNOWN_bits(s_0_1)
        let s_0_2: Bits = u__UNKNOWN_bits(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // C s_0_4: const #90128u : u32
        let s_0_4: u32 = 90128;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<u64>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #64s : i64
        let s_0_6: i64 = 64;
        // C s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // S s_0_8: call __UNKNOWN_bits(s_0_7)
        let s_0_8: Bits = u__UNKNOWN_bits(state, tracer, s_0_7);
        // S s_0_9: cast reint s_0_8 -> u64
        let s_0_9: u64 = (s_0_8.value() as u64);
        // C s_0_10: const #10184u : u32
        let s_0_10: u32 = 10184;
        // N s_0_11: write-reg s_0_10 <= s_0_9
        let s_0_11: () = {
            state.write_register::<u64>(s_0_10 as isize, s_0_9);
            tracer.write_register(s_0_10 as isize, s_0_9);
        };
        // C s_0_12: const #64s : i64
        let s_0_12: i64 = 64;
        // C s_0_13: cast zx s_0_12 -> i
        let s_0_13: i128 = (i128::try_from(s_0_12).unwrap());
        // S s_0_14: call __UNKNOWN_bits(s_0_13)
        let s_0_14: Bits = u__UNKNOWN_bits(state, tracer, s_0_13);
        // S s_0_15: cast reint s_0_14 -> u64
        let s_0_15: u64 = (s_0_14.value() as u64);
        // S s_0_16: call Mk_SPSR_EL1_Type(s_0_15)
        let s_0_16: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL1_Type(
            state,
            tracer,
            s_0_15,
        );
        // C s_0_17: const #90648u : u32
        let s_0_17: u32 = 90648;
        // N s_0_18: write-reg s_0_17 <= s_0_16
        let s_0_18: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_17 as isize, s_0_16);
            tracer.write_register(s_0_17 as isize, s_0_16);
        };
        // C s_0_19: const #64s : i64
        let s_0_19: i64 = 64;
        // C s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // S s_0_21: call __UNKNOWN_bits(s_0_20)
        let s_0_21: Bits = u__UNKNOWN_bits(state, tracer, s_0_20);
        // S s_0_22: cast reint s_0_21 -> u64
        let s_0_22: u64 = (s_0_21.value() as u64);
        // C s_0_23: const #18312u : u32
        let s_0_23: u32 = 18312;
        // N s_0_24: write-reg s_0_23 <= s_0_22
        let s_0_24: () = {
            state.write_register::<u64>(s_0_23 as isize, s_0_22);
            tracer.write_register(s_0_23 as isize, s_0_22);
        };
        // C s_0_25: const #432u : u32
        let s_0_25: u32 = 432;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // C s_0_27: const #2u : u8
        let s_0_27: u8 = 2;
        // D s_0_28: cmp-lt s_0_26 s_0_27
        let s_0_28: bool = ((s_0_26) < (s_0_27));
        // N s_0_29: branch s_0_28 b9 b1
        if s_0_28 {
            return block_9(state, tracer, fn_state);
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
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #2u : u8
        let s_2_2: u8 = 2;
        // D s_2_3: cmp-lt s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) < (s_2_2));
        // N s_2_4: branch s_2_3 b8 b3
        if s_2_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #440u : u32
        let s_4_0: u32 = 440;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call HaveAArch32EL(s_4_1)
        let s_4_2: bool = HaveAArch32EL(state, tracer, s_4_1);
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // S s_6_2: call __UNKNOWN_bits(s_6_1)
        let s_6_2: Bits = u__UNKNOWN_bits(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #13360u : u32
        let s_6_4: u32 = 13360;
        // N s_6_5: write-reg s_6_4 <= s_6_3
        let s_6_5: () = {
            state.write_register::<u64>(s_6_4 as isize, s_6_3);
            tracer.write_register(s_6_4 as isize, s_6_3);
        };
        // C s_6_6: const #64s : i64
        let s_6_6: i64 = 64;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // S s_6_8: call __UNKNOWN_bits(s_6_7)
        let s_6_8: Bits = u__UNKNOWN_bits(state, tracer, s_6_7);
        // S s_6_9: cast reint s_6_8 -> u64
        let s_6_9: u64 = (s_6_8.value() as u64);
        // S s_6_10: call Mk_DSPSR_EL0_Type(s_6_9)
        let s_6_10: ProductType5c790c8ef59cc8b2 = Mk_DSPSR_EL0_Type(
            state,
            tracer,
            s_6_9,
        );
        // C s_6_11: const #102584u : u32
        let s_6_11: u32 = 102584;
        // N s_6_12: write-reg s_6_11 <= s_6_10
        let s_6_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_11 as isize, s_6_10);
            tracer.write_register(s_6_11 as isize, s_6_10);
        };
        // N s_6_13: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #32s : i64
        let s_7_0: i64 = 32;
        // C s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // S s_7_2: call __UNKNOWN_bits(s_7_1)
        let s_7_2: Bits = u__UNKNOWN_bits(state, tracer, s_7_1);
        // C s_7_3: const #15536u : u32
        let s_7_3: u32 = 15536;
        // D s_7_4: read-reg s_7_3:struct
        let s_7_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // C s_7_5: const #15536u : u32
        let s_7_5: u32 = 15536;
        // N s_7_6: write-reg s_7_5 <= s_7_4
        let s_7_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_5 as isize, s_7_4);
            tracer.write_register(s_7_5 as isize, s_7_4);
        };
        // C s_7_7: const #32s : i64
        let s_7_7: i64 = 32;
        // C s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // S s_7_9: call __UNKNOWN_bits(s_7_8)
        let s_7_9: Bits = u__UNKNOWN_bits(state, tracer, s_7_8);
        // C s_7_10: const #91016u : u32
        let s_7_10: u32 = 91016;
        // D s_7_11: read-reg s_7_10:struct
        let s_7_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_10 as isize);
            tracer.read_register(s_7_10 as isize, value);
            value
        };
        // C s_7_12: const #91016u : u32
        let s_7_12: u32 = 91016;
        // N s_7_13: write-reg s_7_12 <= s_7_11
        let s_7_13: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_12 as isize, s_7_11);
            tracer.write_register(s_7_12 as isize, s_7_11);
        };
        // C s_7_14: const #32s : i64
        let s_7_14: i64 = 32;
        // C s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // S s_7_16: call __UNKNOWN_bits(s_7_15)
        let s_7_16: Bits = u__UNKNOWN_bits(state, tracer, s_7_15);
        // C s_7_17: const #20032u : u32
        let s_7_17: u32 = 20032;
        // D s_7_18: read-reg s_7_17:struct
        let s_7_18: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_17 as isize);
            tracer.read_register(s_7_17 as isize, value);
            value
        };
        // C s_7_19: const #20032u : u32
        let s_7_19: u32 = 20032;
        // N s_7_20: write-reg s_7_19 <= s_7_18
        let s_7_20: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_19 as isize, s_7_18);
            tracer.write_register(s_7_19 as isize, s_7_18);
        };
        // C s_7_21: const #32s : i64
        let s_7_21: i64 = 32;
        // C s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // S s_7_23: call __UNKNOWN_bits(s_7_22)
        let s_7_23: Bits = u__UNKNOWN_bits(state, tracer, s_7_22);
        // C s_7_24: const #18424u : u32
        let s_7_24: u32 = 18424;
        // D s_7_25: read-reg s_7_24:struct
        let s_7_25: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_24 as isize);
            tracer.read_register(s_7_24 as isize, value);
            value
        };
        // C s_7_26: const #18424u : u32
        let s_7_26: u32 = 18424;
        // N s_7_27: write-reg s_7_26 <= s_7_25
        let s_7_27: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_7_26 as isize, s_7_25);
            tracer.write_register(s_7_26 as isize, s_7_25);
        };
        // N s_7_28: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // S s_8_2: call __UNKNOWN_bits(s_8_1)
        let s_8_2: Bits = u__UNKNOWN_bits(state, tracer, s_8_1);
        // S s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // C s_8_4: const #12080u : u32
        let s_8_4: u32 = 12080;
        // N s_8_5: write-reg s_8_4 <= s_8_3
        let s_8_5: () = {
            state.write_register::<u64>(s_8_4 as isize, s_8_3);
            tracer.write_register(s_8_4 as isize, s_8_3);
        };
        // C s_8_6: const #64s : i64
        let s_8_6: i64 = 64;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // S s_8_8: call __UNKNOWN_bits(s_8_7)
        let s_8_8: Bits = u__UNKNOWN_bits(state, tracer, s_8_7);
        // S s_8_9: cast reint s_8_8 -> u64
        let s_8_9: u64 = (s_8_8.value() as u64);
        // S s_8_10: call Mk_SPSR_EL3_Type(s_8_9)
        let s_8_10: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL3_Type(state, tracer, s_8_9);
        // C s_8_11: const #20304u : u32
        let s_8_11: u32 = 20304;
        // N s_8_12: write-reg s_8_11 <= s_8_10
        let s_8_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_8_11 as isize, s_8_10);
            tracer.write_register(s_8_11 as isize, s_8_10);
        };
        // C s_8_13: const #64s : i64
        let s_8_13: i64 = 64;
        // C s_8_14: cast zx s_8_13 -> i
        let s_8_14: i128 = (i128::try_from(s_8_13).unwrap());
        // S s_8_15: call __UNKNOWN_bits(s_8_14)
        let s_8_15: Bits = u__UNKNOWN_bits(state, tracer, s_8_14);
        // S s_8_16: cast reint s_8_15 -> u64
        let s_8_16: u64 = (s_8_15.value() as u64);
        // C s_8_17: const #20128u : u32
        let s_8_17: u32 = 20128;
        // N s_8_18: write-reg s_8_17 <= s_8_16
        let s_8_18: () = {
            state.write_register::<u64>(s_8_17 as isize, s_8_16);
            tracer.write_register(s_8_17 as isize, s_8_16);
        };
        // N s_8_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // S s_9_2: call __UNKNOWN_bits(s_9_1)
        let s_9_2: Bits = u__UNKNOWN_bits(state, tracer, s_9_1);
        // S s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // C s_9_4: const #90968u : u32
        let s_9_4: u32 = 90968;
        // N s_9_5: write-reg s_9_4 <= s_9_3
        let s_9_5: () = {
            state.write_register::<u64>(s_9_4 as isize, s_9_3);
            tracer.write_register(s_9_4 as isize, s_9_3);
        };
        // C s_9_6: const #64s : i64
        let s_9_6: i64 = 64;
        // C s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // S s_9_8: call __UNKNOWN_bits(s_9_7)
        let s_9_8: Bits = u__UNKNOWN_bits(state, tracer, s_9_7);
        // S s_9_9: cast reint s_9_8 -> u64
        let s_9_9: u64 = (s_9_8.value() as u64);
        // S s_9_10: call Mk_SPSR_EL2_Type(s_9_9)
        let s_9_10: ProductType5c790c8ef59cc8b2 = Mk_SPSR_EL2_Type(state, tracer, s_9_9);
        // C s_9_11: const #15736u : u32
        let s_9_11: u32 = 15736;
        // N s_9_12: write-reg s_9_11 <= s_9_10
        let s_9_12: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_11 as isize, s_9_10);
            tracer.write_register(s_9_11 as isize, s_9_10);
        };
        // C s_9_13: const #64s : i64
        let s_9_13: i64 = 64;
        // C s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (i128::try_from(s_9_13).unwrap());
        // S s_9_15: call __UNKNOWN_bits(s_9_14)
        let s_9_15: Bits = u__UNKNOWN_bits(state, tracer, s_9_14);
        // S s_9_16: cast reint s_9_15 -> u64
        let s_9_16: u64 = (s_9_15.value() as u64);
        // C s_9_17: const #17224u : u32
        let s_9_17: u32 = 17224;
        // N s_9_18: write-reg s_9_17 <= s_9_16
        let s_9_18: () = {
            state.write_register::<u64>(s_9_17 as isize, s_9_16);
            tracer.write_register(s_9_17 as isize, s_9_16);
        };
        // N s_9_19: jump b2
        return block_2(state, tracer, fn_state);
    }
}
