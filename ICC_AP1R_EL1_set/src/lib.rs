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
use ELUsingAArch32::*;
use CurrentSecurityState::*;
use u__id::*;
use common::*;
pub fn ICC_AP1R_EL1_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_32854: bool,
        r: ProductType5c790c8ef59cc8b2,
        gs_32858: bool,
        gs_32850: bool,
        n: i128,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call ELUsingAArch32(s_0_3)
        let s_0_4: bool = ELUsingAArch32(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b11 b1
        if s_0_4 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#32850 <= s_1_0
        fn_state.gs_32850 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#32850:u8
        let s_2_0: bool = fn_state.gs_32850;
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
        // D s_3_0: read-var n:i
        let s_3_0: i128 = fn_state.n;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cmp-le s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) <= (s_3_1));
        // N s_3_4: branch s_3_3 b6 b4
        if s_3_3 {
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
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#32854 <= s_4_0
        fn_state.gs_32854 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#32854:u8
        let s_5_0: bool = fn_state.gs_32854;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // C s_5_2: const #11536u : u32
        let s_5_2: u32 = 11536;
        // D s_5_3: read-reg s_5_2:[struct; 4]
        let s_5_3: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: read-var n:i
        let s_5_4: i128 = fn_state.n;
        // D s_5_5: read-var r:struct
        let s_5_5: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_5_6: mutate-element s_5_3[s_5_4] <= s_5_5
        let s_5_6: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_5_3.clone();
            local[(s_5_4) as usize] = s_5_5;
            local
        };
        // D s_5_7: cast cvt s_5_6 -> [struct; 0]
        let s_5_7: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_5_6,
        );
        // D s_5_8: cast cvt s_5_7 -> [struct; 4]
        let s_5_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_5_7);
            buf
        };
        // C s_5_9: const #11536u : u32
        let s_5_9: u32 = 11536;
        // N s_5_10: write-reg s_5_9 <= s_5_8
        let s_5_10: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_5_9 as isize, s_5_8);
            tracer.write_register(s_5_9 as isize, s_5_8);
        };
        // N s_5_11: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var n:i
        let s_6_0: i128 = fn_state.n;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // D s_6_4: write-var gs#32854 <= s_6_3
        fn_state.gs_32854 = s_6_3;
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i
        let s_7_0: i128 = fn_state.n;
        // D s_7_1: call __id(s_7_0)
        let s_7_1: i128 = u__id(state, tracer, s_7_0);
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: cmp-le s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) <= (s_7_1));
        // N s_7_4: branch s_7_3 b10 b8
        if s_7_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#32858 <= s_8_0
        fn_state.gs_32858 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#32858:u8
        let s_9_0: bool = fn_state.gs_32858;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // C s_9_2: const #1600u : u32
        let s_9_2: u32 = 1600;
        // D s_9_3: read-reg s_9_2:[struct; 4]
        let s_9_3: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: read-var n:i
        let s_9_4: i128 = fn_state.n;
        // D s_9_5: read-var r:struct
        let s_9_5: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_9_6: mutate-element s_9_3[s_9_4] <= s_9_5
        let s_9_6: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut local = s_9_3.clone();
            local[(s_9_4) as usize] = s_9_5;
            local
        };
        // D s_9_7: cast cvt s_9_6 -> [struct; 0]
        let s_9_7: alloc::vec::Vec<ProductType5c790c8ef59cc8b2> = alloc::vec::Vec::from(
            s_9_6,
        );
        // D s_9_8: cast cvt s_9_7 -> [struct; 4]
        let s_9_8: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_9_7);
            buf
        };
        // C s_9_9: const #1600u : u32
        let s_9_9: u32 = 1600;
        // N s_9_10: write-reg s_9_9 <= s_9_8
        let s_9_10: () = {
            state
                .write_register::<
                    [ProductType5c790c8ef59cc8b2; 4usize],
                >(s_9_9 as isize, s_9_8);
            tracer.write_register(s_9_9 as isize, s_9_8);
        };
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i
        let s_10_0: i128 = fn_state.n;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #4s : i
        let s_10_2: i128 = 4;
        // D s_10_3: cmp-lt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) < (s_10_2));
        // D s_10_4: write-var gs#32858 <= s_10_3
        fn_state.gs_32858 = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call CurrentSecurityState(s_11_0)
        let s_11_1: u32 = CurrentSecurityState(state, tracer, s_11_0);
        // C s_11_2: const #3u : u32
        let s_11_2: u32 = 3;
        // S s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // D s_11_4: write-var gs#32850 <= s_11_3
        fn_state.gs_32850 = s_11_3;
        // N s_11_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
