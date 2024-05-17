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
use SP_read::*;
use BigEndian::*;
use CreateAccDescRCW::*;
use X_read::*;
use CheckSPAlignment::*;
use IsD128Enabled::*;
use MemAtomicRCW::*;
use common::*;
pub fn execute_aarch64_instrs_memory_rcws_cas_128<T: Tracer>(
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
        t2: u64,
        compdata: u128,
        readdata: u128,
        address: u64,
        t1: u64,
        s2: u64,
        accdesc: ProductType9878976b5bcce9c9,
        newdata: u128,
        s1: u64,
        gs_708367: ProductType42c31f3d0ab6eb17,
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
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b15 b1
        if s_0_3 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_5: write-var s1 <= s_1_4
        fn_state.s1 = s_1_4;
        // C s_1_6: const #1s : i
        let s_1_6: i128 = 1;
        // D s_1_7: read-var s:i64
        let s_1_7: i64 = fn_state.s;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: add s_1_8 s_1_6
        let s_1_9: i128 = (s_1_8 + s_1_6);
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // C s_1_11: const #64s : i64
        let s_1_11: i64 = 64;
        // D s_1_12: cast zx s_1_10 -> i
        let s_1_12: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_13: call X_read(s_1_12, s_1_11)
        let s_1_13: Bits = X_read(state, tracer, s_1_12, s_1_11);
        // D s_1_14: cast reint s_1_13 -> u64
        let s_1_14: u64 = (s_1_13.value() as u64);
        // D s_1_15: write-var s2 <= s_1_14
        fn_state.s2 = s_1_14;
        // C s_1_16: const #64s : i64
        let s_1_16: i64 = 64;
        // D s_1_17: read-var t:i64
        let s_1_17: i64 = fn_state.t;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call X_read(s_1_18, s_1_16)
        let s_1_19: Bits = X_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: cast reint s_1_19 -> u64
        let s_1_20: u64 = (s_1_19.value() as u64);
        // D s_1_21: write-var t1 <= s_1_20
        fn_state.t1 = s_1_20;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var t:i64
        let s_1_23: i64 = fn_state.t;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: add s_1_24 s_1_22
        let s_1_25: i128 = (s_1_24 + s_1_22);
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // C s_1_27: const #64s : i64
        let s_1_27: i64 = 64;
        // D s_1_28: cast zx s_1_26 -> i
        let s_1_28: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_29: call X_read(s_1_28, s_1_27)
        let s_1_29: Bits = X_read(state, tracer, s_1_28, s_1_27);
        // D s_1_30: cast reint s_1_29 -> u64
        let s_1_30: u64 = (s_1_29.value() as u64);
        // D s_1_31: write-var t2 <= s_1_30
        fn_state.t2 = s_1_30;
        // C s_1_32: const #10u : u32
        let s_1_32: u32 = 10;
        // D s_1_33: read-var soft:u8
        let s_1_33: bool = fn_state.soft;
        // D s_1_34: read-var acquire:u8
        let s_1_34: bool = fn_state.acquire;
        // D s_1_35: read-var release:u8
        let s_1_35: bool = fn_state.release;
        // D s_1_36: read-var tagchecked:u8
        let s_1_36: bool = fn_state.tagchecked;
        // D s_1_37: call CreateAccDescRCW(s_1_32, s_1_33, s_1_34, s_1_35, s_1_36)
        let s_1_37: ProductType9878976b5bcce9c9 = CreateAccDescRCW(
            state,
            tracer,
            s_1_32,
            s_1_33,
            s_1_34,
            s_1_35,
            s_1_36,
        );
        // D s_1_38: write-var accdesc <= s_1_37
        fn_state.accdesc = s_1_37;
        // D s_1_39: read-var accdesc.1:struct
        let s_1_39: u32 = fn_state.accdesc._1;
        // D s_1_40: call BigEndian(s_1_39)
        let s_1_40: bool = BigEndian(state, tracer, s_1_39);
        // N s_1_41: branch s_1_40 b14 b2
        if s_1_40 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s2:u64
        let s_2_0: u64 = fn_state.s2;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_2: read-var s1:u64
        let s_2_2: u64 = fn_state.s1;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u128
        let s_2_12: u128 = (s_2_11.value() as u128);
        // D s_2_13: write-var compdata <= s_2_12
        fn_state.compdata = s_2_12;
        // N s_2_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var accdesc.1:struct
        let s_3_0: u32 = fn_state.accdesc._1;
        // D s_3_1: call BigEndian(s_3_0)
        let s_3_1: bool = BigEndian(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b13 b4
        if s_3_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var t2:u64
        let s_4_0: u64 = fn_state.t2;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var t1:u64
        let s_4_2: u64 = fn_state.t1;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u128
        let s_4_12: u128 = (s_4_11.value() as u128);
        // D s_4_13: write-var newdata <= s_4_12
        fn_state.newdata = s_4_12;
        // N s_4_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) == (s_5_0));
        // N s_5_4: branch s_5_3 b11 b6
        if s_5_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: call X_read(s_6_2, s_6_0)
        let s_6_3: Bits = X_read(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u64
        let s_6_4: u64 = (s_6_3.value() as u64);
        // D s_6_5: write-var address <= s_6_4
        fn_state.address = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var compdata:u128
        let s_7_0: u128 = fn_state.compdata;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 128u16);
        // D s_7_2: read-var newdata:u128
        let s_7_2: u128 = fn_state.newdata;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 128u16);
        // D s_7_4: read-var address:u64
        let s_7_4: u64 = fn_state.address;
        // D s_7_5: read-var accdesc:struct
        let s_7_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_6: call MemAtomicRCW(s_7_4, s_7_1, s_7_3, s_7_5)
        let s_7_6: ProductType42c31f3d0ab6eb17 = MemAtomicRCW(
            state,
            tracer,
            s_7_4,
            s_7_1,
            s_7_3,
            s_7_5,
        );
        // D s_7_7: write-var gs#708367 <= s_7_6
        fn_state.gs_708367 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#708367.0:struct
        let s_8_0: u8 = fn_state.gs_708367._0;
        // D s_8_1: read-var gs#708367.1:struct
        let s_8_1: Bits = fn_state.gs_708367._1;
        // D s_8_2: cast reint s_8_1 -> u128
        let s_8_2: u128 = (s_8_1.value() as u128);
        // D s_8_3: write-var readdata <= s_8_2
        fn_state.readdata = s_8_2;
        // C s_8_4: const #3s : i
        let s_8_4: i128 = 3;
        // D s_8_5: cast zx s_8_0 -> bv
        let s_8_5: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_6: const #1s : i64
        let s_8_6: i64 = 1;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_8: const #0s : i
        let s_8_8: i128 = 0;
        // C s_8_9: add s_8_8 s_8_7
        let s_8_9: i128 = (s_8_8 + s_8_7);
        // D s_8_10: bit-extract s_8_5 s_8_4 s_8_9
        let s_8_10: Bits = (Bits::new(
            ((s_8_5) >> (s_8_4)).value(),
            u16::try_from(s_8_9).unwrap(),
        ));
        // D s_8_11: cast reint s_8_10 -> u8
        let s_8_11: bool = ((s_8_10.value()) != 0);
        // C s_8_12: const #16984u : u32
        let s_8_12: u32 = 16984;
        // N s_8_13: write-reg s_8_12 <= s_8_11
        let s_8_13: () = {
            state.write_register::<bool>(s_8_12 as isize, s_8_11);
            tracer.write_register(s_8_12 as isize, s_8_11);
        };
        // C s_8_14: const #2s : i
        let s_8_14: i128 = 2;
        // D s_8_15: cast zx s_8_0 -> bv
        let s_8_15: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_16: const #1s : i64
        let s_8_16: i64 = 1;
        // C s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // C s_8_18: const #0s : i
        let s_8_18: i128 = 0;
        // C s_8_19: add s_8_18 s_8_17
        let s_8_19: i128 = (s_8_18 + s_8_17);
        // D s_8_20: bit-extract s_8_15 s_8_14 s_8_19
        let s_8_20: Bits = (Bits::new(
            ((s_8_15) >> (s_8_14)).value(),
            u16::try_from(s_8_19).unwrap(),
        ));
        // D s_8_21: cast reint s_8_20 -> u8
        let s_8_21: bool = ((s_8_20.value()) != 0);
        // C s_8_22: const #16997u : u32
        let s_8_22: u32 = 16997;
        // N s_8_23: write-reg s_8_22 <= s_8_21
        let s_8_23: () = {
            state.write_register::<bool>(s_8_22 as isize, s_8_21);
            tracer.write_register(s_8_22 as isize, s_8_21);
        };
        // C s_8_24: const #1s : i
        let s_8_24: i128 = 1;
        // D s_8_25: cast zx s_8_0 -> bv
        let s_8_25: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_26: const #1s : i64
        let s_8_26: i64 = 1;
        // C s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (i128::try_from(s_8_26).unwrap());
        // C s_8_28: const #0s : i
        let s_8_28: i128 = 0;
        // C s_8_29: add s_8_28 s_8_27
        let s_8_29: i128 = (s_8_28 + s_8_27);
        // D s_8_30: bit-extract s_8_25 s_8_24 s_8_29
        let s_8_30: Bits = (Bits::new(
            ((s_8_25) >> (s_8_24)).value(),
            u16::try_from(s_8_29).unwrap(),
        ));
        // D s_8_31: cast reint s_8_30 -> u8
        let s_8_31: bool = ((s_8_30.value()) != 0);
        // C s_8_32: const #16971u : u32
        let s_8_32: u32 = 16971;
        // N s_8_33: write-reg s_8_32 <= s_8_31
        let s_8_33: () = {
            state.write_register::<bool>(s_8_32 as isize, s_8_31);
            tracer.write_register(s_8_32 as isize, s_8_31);
        };
        // C s_8_34: const #0s : i
        let s_8_34: i128 = 0;
        // D s_8_35: cast zx s_8_0 -> bv
        let s_8_35: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_36: const #1s : i64
        let s_8_36: i64 = 1;
        // C s_8_37: cast zx s_8_36 -> i
        let s_8_37: i128 = (i128::try_from(s_8_36).unwrap());
        // C s_8_38: const #0s : i
        let s_8_38: i128 = 0;
        // C s_8_39: add s_8_38 s_8_37
        let s_8_39: i128 = (s_8_38 + s_8_37);
        // D s_8_40: bit-extract s_8_35 s_8_34 s_8_39
        let s_8_40: Bits = (Bits::new(
            ((s_8_35) >> (s_8_34)).value(),
            u16::try_from(s_8_39).unwrap(),
        ));
        // D s_8_41: cast reint s_8_40 -> u8
        let s_8_41: bool = ((s_8_40.value()) != 0);
        // C s_8_42: const #16996u : u32
        let s_8_42: u32 = 16996;
        // N s_8_43: write-reg s_8_42 <= s_8_41
        let s_8_43: () = {
            state.write_register::<bool>(s_8_42 as isize, s_8_41);
            tracer.write_register(s_8_42 as isize, s_8_41);
        };
        // D s_8_44: read-var accdesc.1:struct
        let s_8_44: u32 = fn_state.accdesc._1;
        // D s_8_45: call BigEndian(s_8_44)
        let s_8_45: bool = BigEndian(state, tracer, s_8_44);
        // N s_8_46: branch s_8_45 b10 b9
        if s_8_45 {
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
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #0s : i
        let s_9_1: i128 = 0;
        // D s_9_2: read-var readdata:u128
        let s_9_2: u128 = fn_state.readdata;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 128u16);
        // C s_9_4: const #1s : i64
        let s_9_4: i64 = 1;
        // C s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // C s_9_6: const #63s : i
        let s_9_6: i128 = 63;
        // C s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: bit-extract s_9_3 s_9_1 s_9_7
        let s_9_8: Bits = (Bits::new(
            ((s_9_3) >> (s_9_1)).value(),
            u16::try_from(s_9_7).unwrap(),
        ));
        // D s_9_9: cast reint s_9_8 -> u64
        let s_9_9: u64 = (s_9_8.value() as u64);
        // D s_9_10: read-var s:i64
        let s_9_10: i64 = fn_state.s;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: cast zx s_9_9 -> bv
        let s_9_12: Bits = Bits::new(s_9_9 as u128, 64u16);
        // D s_9_13: call X_set(s_9_11, s_9_0, s_9_12)
        let s_9_13: () = X_set(state, tracer, s_9_11, s_9_0, s_9_12);
        // C s_9_14: const #1s : i
        let s_9_14: i128 = 1;
        // D s_9_15: read-var s:i64
        let s_9_15: i64 = fn_state.s;
        // D s_9_16: cast zx s_9_15 -> i
        let s_9_16: i128 = (i128::try_from(s_9_15).unwrap());
        // D s_9_17: add s_9_16 s_9_14
        let s_9_17: i128 = (s_9_16 + s_9_14);
        // D s_9_18: cast reint s_9_17 -> i64
        let s_9_18: i64 = (s_9_17 as i64);
        // C s_9_19: const #64s : i64
        let s_9_19: i64 = 64;
        // C s_9_20: const #64s : i
        let s_9_20: i128 = 64;
        // D s_9_21: read-var readdata:u128
        let s_9_21: u128 = fn_state.readdata;
        // D s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 128u16);
        // C s_9_23: const #1s : i64
        let s_9_23: i64 = 1;
        // C s_9_24: cast zx s_9_23 -> i
        let s_9_24: i128 = (i128::try_from(s_9_23).unwrap());
        // C s_9_25: const #63s : i
        let s_9_25: i128 = 63;
        // C s_9_26: add s_9_25 s_9_24
        let s_9_26: i128 = (s_9_25 + s_9_24);
        // D s_9_27: bit-extract s_9_22 s_9_20 s_9_26
        let s_9_27: Bits = (Bits::new(
            ((s_9_22) >> (s_9_20)).value(),
            u16::try_from(s_9_26).unwrap(),
        ));
        // D s_9_28: cast reint s_9_27 -> u64
        let s_9_28: u64 = (s_9_27.value() as u64);
        // D s_9_29: cast zx s_9_18 -> i
        let s_9_29: i128 = (i128::try_from(s_9_18).unwrap());
        // D s_9_30: cast zx s_9_28 -> bv
        let s_9_30: Bits = Bits::new(s_9_28 as u128, 64u16);
        // D s_9_31: call X_set(s_9_29, s_9_19, s_9_30)
        let s_9_31: () = X_set(state, tracer, s_9_29, s_9_19, s_9_30);
        // N s_9_32: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #64s : i
        let s_10_1: i128 = 64;
        // D s_10_2: read-var readdata:u128
        let s_10_2: u128 = fn_state.readdata;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 128u16);
        // C s_10_4: const #1s : i64
        let s_10_4: i64 = 1;
        // C s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // C s_10_6: const #63s : i
        let s_10_6: i128 = 63;
        // C s_10_7: add s_10_6 s_10_5
        let s_10_7: i128 = (s_10_6 + s_10_5);
        // D s_10_8: bit-extract s_10_3 s_10_1 s_10_7
        let s_10_8: Bits = (Bits::new(
            ((s_10_3) >> (s_10_1)).value(),
            u16::try_from(s_10_7).unwrap(),
        ));
        // D s_10_9: cast reint s_10_8 -> u64
        let s_10_9: u64 = (s_10_8.value() as u64);
        // D s_10_10: read-var s:i64
        let s_10_10: i64 = fn_state.s;
        // D s_10_11: cast zx s_10_10 -> i
        let s_10_11: i128 = (i128::try_from(s_10_10).unwrap());
        // D s_10_12: cast zx s_10_9 -> bv
        let s_10_12: Bits = Bits::new(s_10_9 as u128, 64u16);
        // D s_10_13: call X_set(s_10_11, s_10_0, s_10_12)
        let s_10_13: () = X_set(state, tracer, s_10_11, s_10_0, s_10_12);
        // C s_10_14: const #1s : i
        let s_10_14: i128 = 1;
        // D s_10_15: read-var s:i64
        let s_10_15: i64 = fn_state.s;
        // D s_10_16: cast zx s_10_15 -> i
        let s_10_16: i128 = (i128::try_from(s_10_15).unwrap());
        // D s_10_17: add s_10_16 s_10_14
        let s_10_17: i128 = (s_10_16 + s_10_14);
        // D s_10_18: cast reint s_10_17 -> i64
        let s_10_18: i64 = (s_10_17 as i64);
        // C s_10_19: const #64s : i64
        let s_10_19: i64 = 64;
        // C s_10_20: const #0s : i
        let s_10_20: i128 = 0;
        // D s_10_21: read-var readdata:u128
        let s_10_21: u128 = fn_state.readdata;
        // D s_10_22: cast zx s_10_21 -> bv
        let s_10_22: Bits = Bits::new(s_10_21 as u128, 128u16);
        // C s_10_23: const #1s : i64
        let s_10_23: i64 = 1;
        // C s_10_24: cast zx s_10_23 -> i
        let s_10_24: i128 = (i128::try_from(s_10_23).unwrap());
        // C s_10_25: const #63s : i
        let s_10_25: i128 = 63;
        // C s_10_26: add s_10_25 s_10_24
        let s_10_26: i128 = (s_10_25 + s_10_24);
        // D s_10_27: bit-extract s_10_22 s_10_20 s_10_26
        let s_10_27: Bits = (Bits::new(
            ((s_10_22) >> (s_10_20)).value(),
            u16::try_from(s_10_26).unwrap(),
        ));
        // D s_10_28: cast reint s_10_27 -> u64
        let s_10_28: u64 = (s_10_27.value() as u64);
        // D s_10_29: cast zx s_10_18 -> i
        let s_10_29: i128 = (i128::try_from(s_10_18).unwrap());
        // D s_10_30: cast zx s_10_28 -> bv
        let s_10_30: Bits = Bits::new(s_10_28 as u128, 64u16);
        // D s_10_31: call X_set(s_10_29, s_10_19, s_10_30)
        let s_10_31: () = X_set(state, tracer, s_10_29, s_10_19, s_10_30);
        // N s_10_32: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CheckSPAlignment(s_11_0)
        let s_11_1: () = CheckSPAlignment(state, tracer, s_11_0);
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call SP_read(s_12_0)
        let s_12_1: u64 = SP_read(state, tracer, s_12_0);
        // D s_12_2: write-var address <= s_12_1
        fn_state.address = s_12_1;
        // N s_12_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var t1:u64
        let s_13_0: u64 = fn_state.t1;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 64u16);
        // D s_13_2: read-var t2:u64
        let s_13_2: u64 = fn_state.t2;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 64u16);
        // D s_13_4: cast reint s_13_1 -> u128
        let s_13_4: u128 = (s_13_1.value() as u128);
        // D s_13_5: size-of s_13_1
        let s_13_5: u16 = s_13_1.length();
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: lsl s_13_4 s_13_7
        let s_13_8: u128 = s_13_4 << s_13_7;
        // D s_13_9: or s_13_8 s_13_6
        let s_13_9: u128 = ((s_13_8) | (s_13_6));
        // D s_13_10: add s_13_5 s_13_7
        let s_13_10: u16 = (s_13_5 + s_13_7);
        // D s_13_11: create-bits s_13_9 s_13_10
        let s_13_11: Bits = Bits::new(s_13_9, s_13_10);
        // D s_13_12: cast reint s_13_11 -> u128
        let s_13_12: u128 = (s_13_11.value() as u128);
        // D s_13_13: write-var newdata <= s_13_12
        fn_state.newdata = s_13_12;
        // N s_13_14: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var s1:u64
        let s_14_0: u64 = fn_state.s1;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 64u16);
        // D s_14_2: read-var s2:u64
        let s_14_2: u64 = fn_state.s2;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 64u16);
        // D s_14_4: cast reint s_14_1 -> u128
        let s_14_4: u128 = (s_14_1.value() as u128);
        // D s_14_5: size-of s_14_1
        let s_14_5: u16 = s_14_1.length();
        // D s_14_6: cast reint s_14_3 -> u128
        let s_14_6: u128 = (s_14_3.value() as u128);
        // D s_14_7: size-of s_14_3
        let s_14_7: u16 = s_14_3.length();
        // D s_14_8: lsl s_14_4 s_14_7
        let s_14_8: u128 = s_14_4 << s_14_7;
        // D s_14_9: or s_14_8 s_14_6
        let s_14_9: u128 = ((s_14_8) | (s_14_6));
        // D s_14_10: add s_14_5 s_14_7
        let s_14_10: u16 = (s_14_5 + s_14_7);
        // D s_14_11: create-bits s_14_9 s_14_10
        let s_14_11: Bits = Bits::new(s_14_9, s_14_10);
        // D s_14_12: cast reint s_14_11 -> u128
        let s_14_12: u128 = (s_14_11.value() as u128);
        // D s_14_13: write-var compdata <= s_14_12
        fn_state.compdata = s_14_12;
        // N s_14_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
}
