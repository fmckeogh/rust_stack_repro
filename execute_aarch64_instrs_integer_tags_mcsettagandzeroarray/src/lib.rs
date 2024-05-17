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
use u_get_DCZID_EL0_Type_BS::*;
use AArch64_MemTag_set::*;
use SP_read::*;
use CreateAccDescLDGSTG::*;
use Align_bits::*;
use X_read::*;
use Zeros::*;
use CheckSPAlignment::*;
use Mem_set::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcsettagandzeroarray<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        gs_173507: i64,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        i: i64,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        n,
        t,
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
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b11 b1
        if s_0_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: read-var t:i64
        let s_1_1: i64 = fn_state.t;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // C s_1_5: const #0s : i
        let s_1_5: i128 = 0;
        // D s_1_6: cast zx s_1_4 -> bv
        let s_1_6: Bits = Bits::new(s_1_4 as u128, 64u16);
        // C s_1_7: const #1s : i64
        let s_1_7: i64 = 1;
        // C s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // C s_1_9: const #3s : i
        let s_1_9: i128 = 3;
        // C s_1_10: add s_1_9 s_1_8
        let s_1_10: i128 = (s_1_9 + s_1_8);
        // D s_1_11: bit-extract s_1_6 s_1_5 s_1_10
        let s_1_11: Bits = (Bits::new(
            ((s_1_6) >> (s_1_5)).value(),
            u16::try_from(s_1_10).unwrap(),
        ));
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: write-var tag <= s_1_12
        fn_state.tag = s_1_12;
        // C s_1_14: const #31s : i
        let s_1_14: i128 = 31;
        // D s_1_15: read-var n:i64
        let s_1_15: i64 = fn_state.n;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cmp-eq s_1_16 s_1_14
        let s_1_17: bool = ((s_1_16) == (s_1_14));
        // N s_1_18: branch s_1_17 b9 b2
        if s_1_17 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var address <= s_2_4
        fn_state.address = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #102208u : u32
        let s_3_0: u32 = 102208;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_DCZID_EL0_Type_BS(s_3_1)
        let s_3_2: u8 = u_get_DCZID_EL0_Type_BS(state, tracer, s_3_1);
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (s_3_3.value() as i128);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: pow2 s_3_6
        let s_3_7: i128 = (s_3_6).pow(2);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #4s : i
        let s_3_9: i128 = 4;
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: mul s_3_9 s_3_10
        let s_3_11: i128 = ((s_3_9) * (s_3_10));
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var address:u64
        let s_3_13: u64 = fn_state.address;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 64u16);
        // D s_3_15: cast zx s_3_12 -> i
        let s_3_15: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_16: call Align_bits(s_3_14, s_3_15)
        let s_3_16: Bits = Align_bits(state, tracer, s_3_14, s_3_15);
        // D s_3_17: cast reint s_3_16 -> u64
        let s_3_17: u64 = (s_3_16.value() as u64);
        // D s_3_18: write-var address <= s_3_17
        fn_state.address = s_3_17;
        // D s_3_19: cast zx s_3_12 -> i
        let s_3_19: i128 = (i128::try_from(s_3_12).unwrap());
        // C s_3_20: const #456u : u32
        let s_3_20: u32 = 456;
        // D s_3_21: read-reg s_3_20:i64
        let s_3_21: i64 = {
            let value = state.read_register::<i64>(s_3_20 as isize);
            tracer.read_register(s_3_20 as isize, value);
            value
        };
        // D s_3_22: cast zx s_3_21 -> i
        let s_3_22: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_23: lsr s_3_19 s_3_22
        let s_3_23: i128 = s_3_19 >> s_3_22;
        // C s_3_24: const #1u : u32
        let s_3_24: u32 = 1;
        // S s_3_25: call CreateAccDescLDGSTG(s_3_24)
        let s_3_25: ProductType9878976b5bcce9c9 = CreateAccDescLDGSTG(
            state,
            tracer,
            s_3_24,
        );
        // D s_3_26: write-var accdesc <= s_3_25
        fn_state.accdesc = s_3_25;
        // C s_3_27: const #0s : i64
        let s_3_27: i64 = 0;
        // C s_3_28: const #1s : i
        let s_3_28: i128 = 1;
        // D s_3_29: sub s_3_23 s_3_28
        let s_3_29: i128 = ((s_3_23) - (s_3_28));
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: write-var gs#173507 <= s_3_30
        fn_state.gs_173507 = s_3_30;
        // D s_3_32: write-var i <= s_3_27
        fn_state.i = s_3_27;
        // N s_3_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#173507:i64
        let s_4_1: i64 = fn_state.gs_173507;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b8 b5
        if s_4_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var address:u64
        let s_5_0: u64 = fn_state.address;
        // D s_5_1: read-var accdesc:struct
        let s_5_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_2: read-var tag:u8
        let s_5_2: u8 = fn_state.tag;
        // D s_5_3: call AArch64_MemTag_set(s_5_0, s_5_1, s_5_2)
        let s_5_3: () = AArch64_MemTag_set(state, tracer, s_5_0, s_5_1, s_5_2);
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i
        let s_6_0: i128 = 8;
        // C s_6_1: const #1344u : u32
        let s_6_1: u32 = 1344;
        // D s_6_2: read-reg s_6_1:i64
        let s_6_2: i64 = {
            let value = state.read_register::<i64>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: mul s_6_0 s_6_3
        let s_6_4: i128 = ((s_6_0) * (s_6_3));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: call Zeros(s_6_6)
        let s_6_7: Bits = Zeros(state, tracer, s_6_6);
        // D s_6_8: cast reint s_6_7 -> u128
        let s_6_8: u128 = (s_6_7.value() as u128);
        // C s_6_9: const #1344u : u32
        let s_6_9: u32 = 1344;
        // D s_6_10: read-reg s_6_9:i64
        let s_6_10: i64 = {
            let value = state.read_register::<i64>(s_6_9 as isize);
            tracer.read_register(s_6_9 as isize, value);
            value
        };
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: cast zx s_6_8 -> bv
        let s_6_12: Bits = Bits::new(s_6_8 as u128, 128u16);
        // D s_6_13: read-var address:u64
        let s_6_13: u64 = fn_state.address;
        // D s_6_14: read-var accdesc:struct
        let s_6_14: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_15: call Mem_set(s_6_13, s_6_11, s_6_14, s_6_12)
        let s_6_15: () = Mem_set(state, tracer, s_6_13, s_6_11, s_6_14, s_6_12);
        // N s_6_16: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var address:u64
        let s_7_0: u64 = fn_state.address;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // C s_7_2: const #1344u : u32
        let s_7_2: u32 = 1344;
        // D s_7_3: read-reg s_7_2:i64
        let s_7_3: i64 = {
            let value = state.read_register::<i64>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast cvt s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 128);
        // D s_7_6: add s_7_1 s_7_5
        let s_7_6: Bits = (s_7_1 + s_7_5);
        // D s_7_7: cast reint s_7_6 -> u64
        let s_7_7: u64 = (s_7_6.value() as u64);
        // D s_7_8: write-var address <= s_7_7
        fn_state.address = s_7_7;
        // D s_7_9: read-var i:i64
        let s_7_9: i64 = fn_state.i;
        // C s_7_10: const #1s : i64
        let s_7_10: i64 = 1;
        // D s_7_11: add s_7_9 s_7_10
        let s_7_11: i64 = (s_7_9 + s_7_10);
        // D s_7_12: write-var i <= s_7_11
        fn_state.i = s_7_11;
        // N s_7_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CheckSPAlignment(s_9_0)
        let s_9_1: () = CheckSPAlignment(state, tracer, s_9_0);
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call SP_read(s_10_0)
        let s_10_1: u64 = SP_read(state, tracer, s_10_0);
        // D s_10_2: write-var address <= s_10_1
        fn_state.address = s_10_1;
        // N s_10_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
}
