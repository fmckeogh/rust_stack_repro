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
use X_set::*;
use CreateAccDescGPR::*;
use Prefetch::*;
use Mem_read::*;
use SPESampleGeneralPurposeLoadStore::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch64_instrs_memory_literal_general<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    nontemporal: bool,
    offset: u64,
    is_signed: bool,
    size: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        datashadow_1652: Bits,
        address: u64,
        memop: u32,
        nontemporal: bool,
        offset: u64,
        is_signed: bool,
        size: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        memop,
        nontemporal,
        offset,
        is_signed,
        size,
        t,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #12744u : u32
        let s_0_0: u32 = 12744;
        // D s_0_1: read-reg s_0_0:u64
        let s_0_1: u64 = {
            let value = state.read_register::<u64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
        // D s_0_3: read-var offset:u64
        let s_0_3: u64 = fn_state.offset;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 64u16);
        // D s_0_5: add s_0_2 s_0_4
        let s_0_5: Bits = (s_0_2 + s_0_4);
        // D s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: write-var address <= s_0_6
        fn_state.address = s_0_6;
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
        // D s_0_14: cmp-ne s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) != (s_0_13));
        // D s_0_15: read-var memop:u32
        let s_0_15: u32 = fn_state.memop;
        // D s_0_16: read-var nontemporal:u8
        let s_0_16: bool = fn_state.nontemporal;
        // D s_0_17: read-var tagchecked:u8
        let s_0_17: bool = fn_state.tagchecked;
        // D s_0_18: call CreateAccDescGPR(s_0_15, s_0_16, s_0_14, s_0_17)
        let s_0_18: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_15,
            s_0_16,
            s_0_14,
            s_0_17,
        );
        // D s_0_19: write-var accdesc <= s_0_18
        fn_state.accdesc = s_0_18;
        // C s_0_20: const #0u : u32
        let s_0_20: u32 = 0;
        // D s_0_21: read-var memop:u32
        let s_0_21: u32 = fn_state.memop;
        // D s_0_22: cmp-eq s_0_20 s_0_21
        let s_0_22: bool = ((s_0_20) == (s_0_21));
        // D s_0_23: not s_0_22
        let s_0_23: bool = !s_0_22;
        // N s_0_24: branch s_0_23 b8 b1
        if s_0_23 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:i64
        let s_1_0: i64 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: read-var address:u64
        let s_1_2: u64 = fn_state.address;
        // D s_1_3: read-var accdesc:struct
        let s_1_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_4: call Mem_read(s_1_2, s_1_1, s_1_3)
        let s_1_4: Bits = Mem_read(state, tracer, s_1_2, s_1_1, s_1_3);
        // D s_1_5: write-var datashadow#1652 <= s_1_4
        fn_state.datashadow_1652 = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var is_signed:u8
        let s_2_0: bool = fn_state.is_signed;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var size:i64
        let s_3_1: i64 = fn_state.size;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) * (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var t:i64
        let s_3_7: i64 = fn_state.t;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: read-var datashadow#1652:bv
        let s_3_9: Bits = fn_state.datashadow_1652;
        // D s_3_10: call X_set(s_3_8, s_3_6, s_3_9)
        let s_3_10: () = X_set(state, tracer, s_3_8, s_3_6, s_3_9);
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #22416u : u32
        let s_4_0: u32 = 22416;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: bool = {
            let value = state.read_register::<bool>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SPESampleGeneralPurposeLoadStore(s_6_0)
        let s_6_1: () = SPESampleGeneralPurposeLoadStore(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // D s_7_2: read-var datashadow#1652:bv
        let s_7_2: Bits = fn_state.datashadow_1652;
        // D s_7_3: bits-cast sx s_7_2 -> bv length s_7_1
        let s_7_3: Bits = s_7_2.sign_extend(s_7_1);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: read-var t:i64
        let s_7_5: i64 = fn_state.t;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast zx s_7_4 -> bv
        let s_7_7: Bits = Bits::new(s_7_4 as u128, 64u16);
        // D s_7_8: call X_set(s_7_6, s_7_0, s_7_7)
        let s_7_8: () = X_set(state, tracer, s_7_6, s_7_0, s_7_7);
        // N s_7_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // D s_8_1: read-var memop:u32
        let s_8_1: u32 = fn_state.memop;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i
        let s_9_0: i128 = 4;
        // C s_9_1: const #0s : i
        let s_9_1: i128 = 0;
        // D s_9_2: read-var t:i64
        let s_9_2: i64 = fn_state.t;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: call integer_subrange(s_9_3, s_9_0, s_9_1)
        let s_9_4: Bits = integer_subrange(state, tracer, s_9_3, s_9_0, s_9_1);
        // D s_9_5: cast reint s_9_4 -> u8
        let s_9_5: u8 = (s_9_4.value() as u8);
        // D s_9_6: read-var address:u64
        let s_9_6: u64 = fn_state.address;
        // D s_9_7: call Prefetch(s_9_6, s_9_5)
        let s_9_7: () = Prefetch(state, tracer, s_9_6, s_9_5);
        // N s_9_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b4
        return block_4(state, tracer, fn_state);
    }
}
