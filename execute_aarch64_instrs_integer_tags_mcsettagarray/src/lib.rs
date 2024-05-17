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
use u__id::*;
use AArch64_MemTag_set::*;
use CreateAccDescLDGSTG::*;
use Align_bits::*;
use SP_read::*;
use X_read::*;
use CheckSPAlignment::*;
use u_get_GMID_EL1_Type_BS::*;
use common::*;
pub fn execute_aarch64_instrs_integer_tags_mcsettagarray<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_173237: bool,
        data: u64,
        index: i128,
        address: u64,
        accdesc: ProductType9878976b5bcce9c9,
        indexshadow_1982: i128,
        i: i64,
        gs_173228: i64,
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
        // N s_0_7: branch s_0_6 b13 b1
        if s_0_6 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_5: write-var data <= s_1_4
        fn_state.data = s_1_4;
        // C s_1_6: const #31s : i
        let s_1_6: i128 = 31;
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cmp-eq s_1_8 s_1_6
        let s_1_9: bool = ((s_1_8) == (s_1_6));
        // N s_1_10: branch s_1_9 b11 b2
        if s_1_9 {
            return block_11(state, tracer, fn_state);
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
        // C s_3_0: const #100816u : u32
        let s_3_0: u32 = 100816;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_GMID_EL1_Type_BS(s_3_1)
        let s_3_2: u8 = u_get_GMID_EL1_Type_BS(state, tracer, s_3_1);
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
        // C s_3_24: const #4s : i
        let s_3_24: i128 = 4;
        // D s_3_25: read-var address:u64
        let s_3_25: u64 = fn_state.address;
        // D s_3_26: cast zx s_3_25 -> bv
        let s_3_26: Bits = Bits::new(s_3_25 as u128, 64u16);
        // C s_3_27: const #456u : u32
        let s_3_27: u32 = 456;
        // D s_3_28: read-reg s_3_27:i64
        let s_3_28: i64 = {
            let value = state.read_register::<i64>(s_3_27 as isize);
            tracer.read_register(s_3_27 as isize, value);
            value
        };
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: bit-extract s_3_26 s_3_29 s_3_24
        let s_3_30: Bits = (Bits::new(
            ((s_3_26) >> (s_3_29)).value(),
            u16::try_from(s_3_24).unwrap(),
        ));
        // D s_3_31: cast reint s_3_30 -> u8
        let s_3_31: u8 = (s_3_30.value() as u8);
        // D s_3_32: cast zx s_3_31 -> bv
        let s_3_32: Bits = Bits::new(s_3_31 as u128, 4u16);
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (s_3_32.value() as i128);
        // D s_3_34: write-var index <= s_3_33
        fn_state.index = s_3_33;
        // C s_3_35: const #1u : u32
        let s_3_35: u32 = 1;
        // S s_3_36: call CreateAccDescLDGSTG(s_3_35)
        let s_3_36: ProductType9878976b5bcce9c9 = CreateAccDescLDGSTG(
            state,
            tracer,
            s_3_35,
        );
        // D s_3_37: write-var accdesc <= s_3_36
        fn_state.accdesc = s_3_36;
        // C s_3_38: const #0s : i64
        let s_3_38: i64 = 0;
        // C s_3_39: const #1s : i
        let s_3_39: i128 = 1;
        // D s_3_40: sub s_3_23 s_3_39
        let s_3_40: i128 = ((s_3_23) - (s_3_39));
        // D s_3_41: cast reint s_3_40 -> i64
        let s_3_41: i64 = (s_3_40 as i64);
        // D s_3_42: write-var gs#173228 <= s_3_41
        fn_state.gs_173228 = s_3_41;
        // D s_3_43: write-var i <= s_3_38
        fn_state.i = s_3_38;
        // N s_3_44: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#173228:i64
        let s_4_1: i64 = fn_state.gs_173228;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var index:i
        let s_5_0: i128 = fn_state.index;
        // D s_5_1: write-var indexshadow#1982 <= s_5_0
        fn_state.indexshadow_1982 = s_5_0;
        // D s_5_2: read-var indexshadow#1982:i
        let s_5_2: i128 = fn_state.indexshadow_1982;
        // D s_5_3: call __id(s_5_2)
        let s_5_3: i128 = u__id(state, tracer, s_5_2);
        // C s_5_4: const #4s : i
        let s_5_4: i128 = 4;
        // D s_5_5: mul s_5_3 s_5_4
        let s_5_5: i128 = ((s_5_3) * (s_5_4));
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: cmp-le s_5_6 s_5_5
        let s_5_7: bool = ((s_5_6) <= (s_5_5));
        // N s_5_8: branch s_5_7 b9 b6
        if s_5_7 {
            return block_9(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#173237 <= s_6_0
        fn_state.gs_173237 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#173237:u8
        let s_7_0: bool = fn_state.gs_173237;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // C s_7_2: const #4s : i
        let s_7_2: i128 = 4;
        // D s_7_3: read-var indexshadow#1982:i
        let s_7_3: i128 = fn_state.indexshadow_1982;
        // D s_7_4: mul s_7_3 s_7_2
        let s_7_4: i128 = ((s_7_3) * (s_7_2));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // C s_7_6: const #4s : i
        let s_7_6: i128 = 4;
        // D s_7_7: read-var data:u64
        let s_7_7: u64 = fn_state.data;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 64u16);
        // D s_7_9: cast zx s_7_5 -> i
        let s_7_9: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_10: bit-extract s_7_8 s_7_9 s_7_6
        let s_7_10: Bits = (Bits::new(
            ((s_7_8) >> (s_7_9)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_11: cast reint s_7_10 -> u8
        let s_7_11: u8 = (s_7_10.value() as u8);
        // D s_7_12: read-var address:u64
        let s_7_12: u64 = fn_state.address;
        // D s_7_13: read-var accdesc:struct
        let s_7_13: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_14: call AArch64_MemTag_set(s_7_12, s_7_13, s_7_11)
        let s_7_14: () = AArch64_MemTag_set(state, tracer, s_7_12, s_7_13, s_7_11);
        // N s_7_15: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var address:u64
        let s_8_0: u64 = fn_state.address;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // C s_8_2: const #1344u : u32
        let s_8_2: u32 = 1344;
        // D s_8_3: read-reg s_8_2:i64
        let s_8_3: i64 = {
            let value = state.read_register::<i64>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast cvt s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 128);
        // D s_8_6: add s_8_1 s_8_5
        let s_8_6: Bits = (s_8_1 + s_8_5);
        // D s_8_7: cast reint s_8_6 -> u64
        let s_8_7: u64 = (s_8_6.value() as u64);
        // D s_8_8: write-var address <= s_8_7
        fn_state.address = s_8_7;
        // C s_8_9: const #1s : i
        let s_8_9: i128 = 1;
        // D s_8_10: read-var index:i
        let s_8_10: i128 = fn_state.index;
        // D s_8_11: add s_8_10 s_8_9
        let s_8_11: i128 = (s_8_10 + s_8_9);
        // D s_8_12: write-var index <= s_8_11
        fn_state.index = s_8_11;
        // D s_8_13: read-var i:i64
        let s_8_13: i64 = fn_state.i;
        // C s_8_14: const #1s : i64
        let s_8_14: i64 = 1;
        // D s_8_15: add s_8_13 s_8_14
        let s_8_15: i64 = (s_8_13 + s_8_14);
        // D s_8_16: write-var i <= s_8_15
        fn_state.i = s_8_15;
        // N s_8_17: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var indexshadow#1982:i
        let s_9_0: i128 = fn_state.indexshadow_1982;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #4s : i
        let s_9_2: i128 = 4;
        // D s_9_3: mul s_9_1 s_9_2
        let s_9_3: i128 = ((s_9_1) * (s_9_2));
        // C s_9_4: const #3s : i
        let s_9_4: i128 = 3;
        // D s_9_5: add s_9_3 s_9_4
        let s_9_5: i128 = (s_9_3 + s_9_4);
        // C s_9_6: const #64s : i
        let s_9_6: i128 = 64;
        // D s_9_7: cmp-lt s_9_5 s_9_6
        let s_9_7: bool = ((s_9_5) < (s_9_6));
        // D s_9_8: write-var gs#173237 <= s_9_7
        fn_state.gs_173237 = s_9_7;
        // N s_9_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
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
        // N s_12_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
}
