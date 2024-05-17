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
use Mem_set::*;
use CheckGCSSTREnabled::*;
use X_read::*;
use AArch64_IsUnprivAccessPriv::*;
use CheckSPAlignment::*;
use SP_read::*;
use CreateAccDescGCS::*;
use common::*;
pub fn execute_aarch64_instrs_memory_gcs_general_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    is_gcssttr: bool,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        effective_el: u8,
        gs_155004: bool,
        address: u64,
        is_gcssttr: bool,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        is_gcssttr,
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
        // D s_0_0: read-var is_gcssttr:u8
        let s_0_0: bool = fn_state.is_gcssttr;
        // D s_0_1: not s_0_0
        let s_0_1: bool = !s_0_0;
        // N s_0_2: branch s_0_1 b13 b1
        if s_0_1 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call AArch64_IsUnprivAccessPriv(s_1_0)
        let s_1_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_1_0);
        // D s_1_2: write-var gs#155004 <= s_1_1
        fn_state.gs_155004 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#155004:u8
        let s_2_0: bool = fn_state.gs_155004;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #448u : u32
        let s_3_0: u32 = 448;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: write-var effective_el <= s_3_1
        fn_state.effective_el = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: read-var effective_el:u8
        let s_4_2: u8 = fn_state.effective_el;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // D s_4_4: cast zx s_4_1 -> bv
        let s_4_4: Bits = Bits::new(s_4_1 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_3 s_4_4
        let s_4_5: bool = ((s_4_3) == (s_4_4));
        // N s_4_6: branch s_4_5 b11 b5
        if s_4_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_6_0: read-var effective_el:u8
        let s_6_0: u8 = fn_state.effective_el;
        // C s_6_1: const #1u : u32
        let s_6_1: u32 = 1;
        // D s_6_2: call CreateAccDescGCS(s_6_0, s_6_1)
        let s_6_2: ProductType9878976b5bcce9c9 = CreateAccDescGCS(
            state,
            tracer,
            s_6_0,
            s_6_1,
        );
        // D s_6_3: write-var accdesc <= s_6_2
        fn_state.accdesc = s_6_2;
        // C s_6_4: const #31s : i
        let s_6_4: i128 = 31;
        // D s_6_5: read-var n:i64
        let s_6_5: i64 = fn_state.n;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: cmp-eq s_6_6 s_6_4
        let s_6_7: bool = ((s_6_6) == (s_6_4));
        // N s_6_8: branch s_6_7 b9 b7
        if s_6_7 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var n:i64
        let s_7_1: i64 = fn_state.n;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: call X_read(s_7_2, s_7_0)
        let s_7_3: Bits = X_read(state, tracer, s_7_2, s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: write-var address <= s_7_4
        fn_state.address = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // D s_8_1: read-var t:i64
        let s_8_1: i64 = fn_state.t;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: call X_read(s_8_2, s_8_0)
        let s_8_3: Bits = X_read(state, tracer, s_8_2, s_8_0);
        // D s_8_4: cast reint s_8_3 -> u64
        let s_8_4: u64 = (s_8_3.value() as u64);
        // C s_8_5: const #8s : i
        let s_8_5: i128 = 8;
        // D s_8_6: cast zx s_8_4 -> bv
        let s_8_6: Bits = Bits::new(s_8_4 as u128, 64u16);
        // D s_8_7: read-var address:u64
        let s_8_7: u64 = fn_state.address;
        // D s_8_8: read-var accdesc:struct
        let s_8_8: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_9: call Mem_set(s_8_7, s_8_5, s_8_8, s_8_6)
        let s_8_9: () = Mem_set(state, tracer, s_8_7, s_8_5, s_8_8, s_8_6);
        // N s_8_10: return
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
        // N s_10_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CheckGCSSTREnabled(s_11_0)
        let s_11_1: () = CheckGCSSTREnabled(state, tracer, s_11_0);
        // N s_11_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #16975u : u32
        let s_12_0: u32 = 16975;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var effective_el <= s_12_1
        fn_state.effective_el = s_12_1;
        // N s_12_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#155004 <= s_13_0
        fn_state.gs_155004 = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
