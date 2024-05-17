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
use u__UNKNOWN_bits::*;
use SP_read::*;
use CreateAccDescRCW::*;
use X_read::*;
use CheckSPAlignment::*;
use IsD128Enabled::*;
use MemAtomicRCW::*;
use common::*;
pub fn execute_aarch64_instrs_memory_rcw_swp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acquire: bool,
    n: i64,
    release: bool,
    s: i64,
    soft: bool,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_708853: ProductType42c31f3d0ab6eb17,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        newdata: u64,
        acquire: bool,
        n: i64,
        release: bool,
        s: i64,
        soft: bool,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        acquire,
        n,
        release,
        s,
        soft,
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
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call IsD128Enabled(s_0_1)
        let s_0_2: bool = IsD128Enabled(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b7 b1
        if s_0_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_1_1: read-var s:i64
        let s_1_1: i64 = fn_state.s;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call X_read(s_1_2, s_1_0)
        let s_1_3: Bits = X_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u64
        let s_1_4: u64 = (s_1_3.value() as u64);
        // D s_1_5: write-var newdata <= s_1_4
        fn_state.newdata = s_1_4;
        // C s_1_6: const #9u : u32
        let s_1_6: u32 = 9;
        // D s_1_7: read-var soft:u8
        let s_1_7: bool = fn_state.soft;
        // D s_1_8: read-var acquire:u8
        let s_1_8: bool = fn_state.acquire;
        // D s_1_9: read-var release:u8
        let s_1_9: bool = fn_state.release;
        // D s_1_10: read-var tagchecked:u8
        let s_1_10: bool = fn_state.tagchecked;
        // D s_1_11: call CreateAccDescRCW(s_1_6, s_1_7, s_1_8, s_1_9, s_1_10)
        let s_1_11: ProductType9878976b5bcce9c9 = CreateAccDescRCW(
            state,
            tracer,
            s_1_6,
            s_1_7,
            s_1_8,
            s_1_9,
            s_1_10,
        );
        // D s_1_12: write-var accdesc <= s_1_11
        fn_state.accdesc = s_1_11;
        // C s_1_13: const #31s : i
        let s_1_13: i128 = 31;
        // D s_1_14: read-var n:i64
        let s_1_14: i64 = fn_state.n;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cmp-eq s_1_15 s_1_13
        let s_1_16: bool = ((s_1_15) == (s_1_13));
        // N s_1_17: branch s_1_16 b5 b2
        if s_1_16 {
            return block_5(state, tracer, fn_state);
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
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // C s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // S s_3_2: call __UNKNOWN_bits(s_3_1)
        let s_3_2: Bits = u__UNKNOWN_bits(state, tracer, s_3_1);
        // S s_3_3: cast reint s_3_2 -> u64
        let s_3_3: u64 = (s_3_2.value() as u64);
        // S s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_5: read-var newdata:u64
        let s_3_5: u64 = fn_state.newdata;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 64u16);
        // D s_3_7: read-var address:u64
        let s_3_7: u64 = fn_state.address;
        // D s_3_8: read-var accdesc:struct
        let s_3_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_9: call MemAtomicRCW(s_3_7, s_3_4, s_3_6, s_3_8)
        let s_3_9: ProductType42c31f3d0ab6eb17 = MemAtomicRCW(
            state,
            tracer,
            s_3_7,
            s_3_4,
            s_3_6,
            s_3_8,
        );
        // D s_3_10: write-var gs#708853 <= s_3_9
        fn_state.gs_708853 = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#708853.0:struct
        let s_4_0: u8 = fn_state.gs_708853._0;
        // D s_4_1: read-var gs#708853.1:struct
        let s_4_1: Bits = fn_state.gs_708853._1;
        // D s_4_2: cast reint s_4_1 -> u64
        let s_4_2: u64 = (s_4_1.value() as u64);
        // C s_4_3: const #3s : i
        let s_4_3: i128 = 3;
        // D s_4_4: cast zx s_4_0 -> bv
        let s_4_4: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #0s : i
        let s_4_7: i128 = 0;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_3 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_3)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u8
        let s_4_10: bool = ((s_4_9.value()) != 0);
        // C s_4_11: const #16984u : u32
        let s_4_11: u32 = 16984;
        // N s_4_12: write-reg s_4_11 <= s_4_10
        let s_4_12: () = {
            state.write_register::<bool>(s_4_11 as isize, s_4_10);
            tracer.write_register(s_4_11 as isize, s_4_10);
        };
        // C s_4_13: const #2s : i
        let s_4_13: i128 = 2;
        // D s_4_14: cast zx s_4_0 -> bv
        let s_4_14: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_15: const #1s : i64
        let s_4_15: i64 = 1;
        // C s_4_16: cast zx s_4_15 -> i
        let s_4_16: i128 = (i128::try_from(s_4_15).unwrap());
        // C s_4_17: const #0s : i
        let s_4_17: i128 = 0;
        // C s_4_18: add s_4_17 s_4_16
        let s_4_18: i128 = (s_4_17 + s_4_16);
        // D s_4_19: bit-extract s_4_14 s_4_13 s_4_18
        let s_4_19: Bits = (Bits::new(
            ((s_4_14) >> (s_4_13)).value(),
            u16::try_from(s_4_18).unwrap(),
        ));
        // D s_4_20: cast reint s_4_19 -> u8
        let s_4_20: bool = ((s_4_19.value()) != 0);
        // C s_4_21: const #16997u : u32
        let s_4_21: u32 = 16997;
        // N s_4_22: write-reg s_4_21 <= s_4_20
        let s_4_22: () = {
            state.write_register::<bool>(s_4_21 as isize, s_4_20);
            tracer.write_register(s_4_21 as isize, s_4_20);
        };
        // C s_4_23: const #1s : i
        let s_4_23: i128 = 1;
        // D s_4_24: cast zx s_4_0 -> bv
        let s_4_24: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_25: const #1s : i64
        let s_4_25: i64 = 1;
        // C s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // C s_4_27: const #0s : i
        let s_4_27: i128 = 0;
        // C s_4_28: add s_4_27 s_4_26
        let s_4_28: i128 = (s_4_27 + s_4_26);
        // D s_4_29: bit-extract s_4_24 s_4_23 s_4_28
        let s_4_29: Bits = (Bits::new(
            ((s_4_24) >> (s_4_23)).value(),
            u16::try_from(s_4_28).unwrap(),
        ));
        // D s_4_30: cast reint s_4_29 -> u8
        let s_4_30: bool = ((s_4_29.value()) != 0);
        // C s_4_31: const #16971u : u32
        let s_4_31: u32 = 16971;
        // N s_4_32: write-reg s_4_31 <= s_4_30
        let s_4_32: () = {
            state.write_register::<bool>(s_4_31 as isize, s_4_30);
            tracer.write_register(s_4_31 as isize, s_4_30);
        };
        // C s_4_33: const #0s : i
        let s_4_33: i128 = 0;
        // D s_4_34: cast zx s_4_0 -> bv
        let s_4_34: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_35: const #1s : i64
        let s_4_35: i64 = 1;
        // C s_4_36: cast zx s_4_35 -> i
        let s_4_36: i128 = (i128::try_from(s_4_35).unwrap());
        // C s_4_37: const #0s : i
        let s_4_37: i128 = 0;
        // C s_4_38: add s_4_37 s_4_36
        let s_4_38: i128 = (s_4_37 + s_4_36);
        // D s_4_39: bit-extract s_4_34 s_4_33 s_4_38
        let s_4_39: Bits = (Bits::new(
            ((s_4_34) >> (s_4_33)).value(),
            u16::try_from(s_4_38).unwrap(),
        ));
        // D s_4_40: cast reint s_4_39 -> u8
        let s_4_40: bool = ((s_4_39.value()) != 0);
        // C s_4_41: const #16996u : u32
        let s_4_41: u32 = 16996;
        // N s_4_42: write-reg s_4_41 <= s_4_40
        let s_4_42: () = {
            state.write_register::<bool>(s_4_41 as isize, s_4_40);
            tracer.write_register(s_4_41 as isize, s_4_40);
        };
        // C s_4_43: const #64s : i64
        let s_4_43: i64 = 64;
        // D s_4_44: read-var t:i64
        let s_4_44: i64 = fn_state.t;
        // D s_4_45: cast zx s_4_44 -> i
        let s_4_45: i128 = (i128::try_from(s_4_44).unwrap());
        // D s_4_46: cast zx s_4_2 -> bv
        let s_4_46: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_47: call X_set(s_4_45, s_4_43, s_4_46)
        let s_4_47: () = X_set(state, tracer, s_4_45, s_4_43, s_4_46);
        // N s_4_48: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call CheckSPAlignment(s_5_0)
        let s_5_1: () = CheckSPAlignment(state, tracer, s_5_0);
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SP_read(s_6_0)
        let s_6_1: u64 = SP_read(state, tracer, s_6_0);
        // D s_6_2: write-var address <= s_6_1
        fn_state.address = s_6_1;
        // N s_6_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
}
