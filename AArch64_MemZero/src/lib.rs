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
use CreateAccDescDCZero::*;
use u_get_DCZID_EL0_Type_BS::*;
use HaveMTE2Ext::*;
use AArch64_DataMemZero::*;
use Align_bits::*;
use AArch64_TagMemZero::*;
use common::*;
pub fn AArch64_MemZero<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regval: u64,
    cachetype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        vaddress: u64,
        gs_27355: bool,
        gs_27356: bool,
        gs_27354: bool,
        size: i64,
        regval: u64,
        cachetype: u32,
    }
    let fn_state = FunctionState {
        regval,
        cachetype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #102208u : u32
        let s_0_0: u32 = 102208;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_DCZID_EL0_Type_BS(s_0_1)
        let s_0_2: u8 = u_get_DCZID_EL0_Type_BS(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: pow2 s_0_6
        let s_0_7: i128 = (s_0_6).pow(2);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // C s_0_9: const #4s : i
        let s_0_9: i128 = 4;
        // D s_0_10: cast zx s_0_8 -> i
        let s_0_10: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_11: mul s_0_9 s_0_10
        let s_0_11: i128 = ((s_0_9) * (s_0_10));
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: write-var size <= s_0_12
        fn_state.size = s_0_12;
        // D s_0_14: read-var size:i64
        let s_0_14: i64 = fn_state.size;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // C s_0_16: const #1088u : u32
        let s_0_16: u32 = 1088;
        // D s_0_17: read-reg s_0_16:i64
        let s_0_17: i64 = {
            let value = state.read_register::<i64>(s_0_16 as isize);
            tracer.read_register(s_0_16 as isize, value);
            value
        };
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: cmp-le s_0_15 s_0_18
        let s_0_19: bool = ((s_0_15) <= (s_0_18));
        // N s_0_20: assert s_0_19
        let s_0_20: () = assert!(s_0_19);
        // C s_0_21: const #() : ()
        let s_0_21: () = ();
        // S s_0_22: call HaveMTE2Ext(s_0_21)
        let s_0_22: bool = HaveMTE2Ext(state, tracer, s_0_21);
        // N s_0_23: branch s_0_22 b17 b1
        if s_0_22 {
            return block_17(state, tracer, fn_state);
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
        // D s_2_0: read-var regval:u64
        let s_2_0: u64 = fn_state.regval;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_2: read-var size:i64
        let s_2_2: i64 = fn_state.size;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call Align_bits(s_2_1, s_2_3)
        let s_2_4: Bits = Align_bits(state, tracer, s_2_1, s_2_3);
        // D s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // D s_2_6: write-var vaddress <= s_2_5
        fn_state.vaddress = s_2_5;
        // D s_2_7: read-var cachetype:u32
        let s_2_7: u32 = fn_state.cachetype;
        // C s_2_8: const #1u : u32
        let s_2_8: u32 = 1;
        // D s_2_9: cmp-eq s_2_7 s_2_8
        let s_2_9: bool = ((s_2_7) == (s_2_8));
        // N s_2_10: branch s_2_9 b16 b3
        if s_2_9 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var cachetype:u32
        let s_3_0: u32 = fn_state.cachetype;
        // C s_3_1: const #2u : u32
        let s_3_1: u32 = 2;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: write-var gs#27354 <= s_3_2
        fn_state.gs_27354 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#27354:u8
        let s_4_0: bool = fn_state.gs_27354;
        // D s_4_1: read-var cachetype:u32
        let s_4_1: u32 = fn_state.cachetype;
        // C s_4_2: const #0u : u32
        let s_4_2: u32 = 0;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: call CreateAccDescDCZero(s_4_0, s_4_3)
        let s_4_4: ProductType9878976b5bcce9c9 = CreateAccDescDCZero(
            state,
            tracer,
            s_4_0,
            s_4_3,
        );
        // D s_4_5: write-var accdesc <= s_4_4
        fn_state.accdesc = s_4_4;
        // D s_4_6: read-var cachetype:u32
        let s_4_6: u32 = fn_state.cachetype;
        // C s_4_7: const #1u : u32
        let s_4_7: u32 = 1;
        // D s_4_8: cmp-eq s_4_6 s_4_7
        let s_4_8: bool = ((s_4_6) == (s_4_7));
        // N s_4_9: branch s_4_8 b15 b5
        if s_4_8 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var cachetype:u32
        let s_5_0: u32 = fn_state.cachetype;
        // C s_5_1: const #2u : u32
        let s_5_1: u32 = 2;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: write-var gs#27355 <= s_5_2
        fn_state.gs_27355 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#27355:u8
        let s_6_0: bool = fn_state.gs_27355;
        // N s_6_1: branch s_6_0 b14 b7
        if s_6_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var cachetype:u32
        let s_8_0: u32 = fn_state.cachetype;
        // C s_8_1: const #0u : u32
        let s_8_1: u32 = 0;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b13 b9
        if s_8_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var cachetype:u32
        let s_9_0: u32 = fn_state.cachetype;
        // C s_9_1: const #2u : u32
        let s_9_1: u32 = 2;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var gs#27356 <= s_9_2
        fn_state.gs_27356 = s_9_2;
        // N s_9_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#27356:u8
        let s_10_0: bool = fn_state.gs_27356;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var size:i64
        let s_12_0: i64 = fn_state.size;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var regval:u64
        let s_12_2: u64 = fn_state.regval;
        // D s_12_3: read-var vaddress:u64
        let s_12_3: u64 = fn_state.vaddress;
        // D s_12_4: read-var accdesc:struct
        let s_12_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_5: call AArch64_DataMemZero(s_12_2, s_12_3, s_12_4, s_12_1)
        let s_12_5: () = AArch64_DataMemZero(
            state,
            tracer,
            s_12_2,
            s_12_3,
            s_12_4,
            s_12_1,
        );
        // N s_12_6: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#27356 <= s_13_0
        fn_state.gs_27356 = s_13_0;
        // N s_13_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var size:i64
        let s_14_0: i64 = fn_state.size;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var regval:u64
        let s_14_2: u64 = fn_state.regval;
        // D s_14_3: read-var vaddress:u64
        let s_14_3: u64 = fn_state.vaddress;
        // D s_14_4: read-var accdesc:struct
        let s_14_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_5: call AArch64_TagMemZero(s_14_2, s_14_3, s_14_4, s_14_1)
        let s_14_5: () = AArch64_TagMemZero(
            state,
            tracer,
            s_14_2,
            s_14_3,
            s_14_4,
            s_14_1,
        );
        // N s_14_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#27355 <= s_15_0
        fn_state.gs_27355 = s_15_0;
        // N s_15_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#27354 <= s_16_0
        fn_state.gs_27354 = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:i64
        let s_17_0: i64 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // C s_17_2: const #1344u : u32
        let s_17_2: u32 = 1344;
        // D s_17_3: read-reg s_17_2:i64
        let s_17_3: i64 = {
            let value = state.read_register::<i64>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cmp-ge s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) >= (s_17_4));
        // N s_17_6: assert s_17_5
        let s_17_6: () = assert!(s_17_5);
        // N s_17_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
