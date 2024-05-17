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
use u_get_TTBCR_Type_A1::*;
use u_get_CONTEXTIDR_Type_ASID::*;
use u_get_TTBR0_Type_ASID::*;
use u_get_TTBCR_Type_T0SZ::*;
use u_get_TTBCR_Type_T1SZ::*;
use u_get_TTBR0_Type_CnP::*;
use u_get_TTBCR_Type_EAE::*;
use u_get_TTBR1_Type_ASID::*;
use AArch32_GetVARange::*;
use HaveCommonNotPrivateTransExt::*;
use u_get_TTBR1_Type_CnP::*;
use common::*;
pub fn AArch32_TLBContextEL30<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        ga_21505: u8,
        gs_27858: bool,
        va: u32,
    }
    let fn_state = FunctionState {
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_0_0: const #3u : u32
        let s_0_0: u32 = 3;
        // D s_0_1: write-var tlbcontext.11 <= s_0_0
        fn_state.tlbcontext._11 = s_0_0;
        // C s_0_2: const #1u : u32
        let s_0_2: u32 = 1;
        // D s_0_3: write-var tlbcontext.10 <= s_0_2
        fn_state.tlbcontext._10 = s_0_2;
        // C s_0_4: const #15368u : u32
        let s_0_4: u32 = 15368;
        // D s_0_5: read-reg s_0_4:struct
        let s_0_5: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: call _get_TTBCR_Type_EAE(s_0_5)
        let s_0_6: bool = u_get_TTBCR_Type_EAE(state, tracer, s_0_5);
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 1u16);
        // C s_0_8: const #1u : u8
        let s_0_8: bool = true;
        // C s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 1u16);
        // D s_0_10: cmp-eq s_0_7 s_0_9
        let s_0_10: bool = ((s_0_7) == (s_0_9));
        // N s_0_11: branch s_0_10 b11 b1
        if s_0_10 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_1_0: const #21936u : u32
        let s_1_0: u32 = 21936;
        // D s_1_1: read-reg s_1_0:struct
        let s_1_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call _get_CONTEXTIDR_Type_ASID(s_1_1)
        let s_1_2: u8 = u_get_CONTEXTIDR_Type_ASID(state, tracer, s_1_1);
        // C s_1_3: const #16s : i
        let s_1_3: i128 = 16;
        // D s_1_4: cast zx s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 8u16);
        // D s_1_5: bits-cast zx s_1_4 -> bv length s_1_3
        let s_1_5: Bits = s_1_4.zero_extend(s_1_3);
        // D s_1_6: cast reint s_1_5 -> u16
        let s_1_6: u16 = (s_1_5.value() as u16);
        // D s_1_7: write-var tlbcontext.0 <= s_1_6
        fn_state.tlbcontext._0 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var tlbcontext.12 <= s_2_0
        fn_state.tlbcontext._12 = s_2_0;
        // C s_2_2: const #64s : i
        let s_2_2: i128 = 64;
        // D s_2_3: read-var va:u32
        let s_2_3: u32 = fn_state.va;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 32u16);
        // D s_2_5: bits-cast zx s_2_4 -> bv length s_2_2
        let s_2_5: Bits = s_2_4.zero_extend(s_2_2);
        // D s_2_6: cast reint s_2_5 -> u64
        let s_2_6: u64 = (s_2_5.value() as u64);
        // D s_2_7: write-var tlbcontext.2 <= s_2_6
        fn_state.tlbcontext._2 = s_2_6;
        // C s_2_8: const #() : ()
        let s_2_8: () = ();
        // S s_2_9: call HaveCommonNotPrivateTransExt(s_2_8)
        let s_2_9: bool = HaveCommonNotPrivateTransExt(state, tracer, s_2_8);
        // N s_2_10: branch s_2_9 b10 b3
        if s_2_9 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#27858 <= s_3_0
        fn_state.gs_27858 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_4_0: read-var gs#27858:u8
        let s_4_0: bool = fn_state.gs_27858;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var tlbcontext.1 <= s_5_0
        fn_state.tlbcontext._1 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_6_0: read-var tlbcontext:struct
        let s_6_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_7_0: const #15368u : u32
        let s_7_0: u32 = 15368;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_TTBCR_Type_T0SZ(s_7_1)
        let s_7_2: u8 = u_get_TTBCR_Type_T0SZ(state, tracer, s_7_1);
        // C s_7_3: const #15368u : u32
        let s_7_3: u32 = 15368;
        // D s_7_4: read-reg s_7_3:struct
        let s_7_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: call _get_TTBCR_Type_T1SZ(s_7_4)
        let s_7_5: u8 = u_get_TTBCR_Type_T1SZ(state, tracer, s_7_4);
        // D s_7_6: read-var va:u32
        let s_7_6: u32 = fn_state.va;
        // D s_7_7: call AArch32_GetVARange(s_7_6, s_7_2, s_7_5)
        let s_7_7: u32 = AArch32_GetVARange(state, tracer, s_7_6, s_7_2, s_7_5);
        // C s_7_8: const #0u : u32
        let s_7_8: u32 = 0;
        // D s_7_9: cmp-eq s_7_7 s_7_8
        let s_7_9: bool = ((s_7_7) == (s_7_8));
        // N s_7_10: branch s_7_9 b9 b8
        if s_7_9 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_8_0: const #19120u : u32
        let s_8_0: u32 = 19120;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_TTBR1_Type_CnP(s_8_1)
        let s_8_2: bool = u_get_TTBR1_Type_CnP(state, tracer, s_8_1);
        // D s_8_3: write-var tlbcontext.1 <= s_8_2
        fn_state.tlbcontext._1 = s_8_2;
        // N s_8_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_9_0: const #101800u : u32
        let s_9_0: u32 = 101800;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_TTBR0_Type_CnP(s_9_1)
        let s_9_2: bool = u_get_TTBR0_Type_CnP(state, tracer, s_9_1);
        // D s_9_3: write-var tlbcontext.1 <= s_9_2
        fn_state.tlbcontext._1 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_10_0: const #15368u : u32
        let s_10_0: u32 = 15368;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_TTBCR_Type_EAE(s_10_1)
        let s_10_2: bool = u_get_TTBCR_Type_EAE(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#27858 <= s_10_6
        fn_state.gs_27858 = s_10_6;
        // N s_10_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_11_0: const #15368u : u32
        let s_11_0: u32 = 15368;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_TTBCR_Type_A1(s_11_1)
        let s_11_2: bool = u_get_TTBCR_Type_A1(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #0u : u8
        let s_11_4: bool = false;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b14 b12
        if s_11_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_12_0: const #19120u : u32
        let s_12_0: u32 = 19120;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_TTBR1_Type_ASID(s_12_1)
        let s_12_2: u8 = u_get_TTBR1_Type_ASID(state, tracer, s_12_1);
        // D s_12_3: write-var ga#21505 <= s_12_2
        fn_state.ga_21505 = s_12_2;
        // N s_12_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_13_0: const #16s : i
        let s_13_0: i128 = 16;
        // D s_13_1: read-var ga#21505:u8
        let s_13_1: u8 = fn_state.ga_21505;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 8u16);
        // D s_13_3: bits-cast zx s_13_2 -> bv length s_13_0
        let s_13_3: Bits = s_13_2.zero_extend(s_13_0);
        // D s_13_4: cast reint s_13_3 -> u16
        let s_13_4: u16 = (s_13_3.value() as u16);
        // D s_13_5: write-var tlbcontext.0 <= s_13_4
        fn_state.tlbcontext._0 = s_13_4;
        // N s_13_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_14_0: const #101800u : u32
        let s_14_0: u32 = 101800;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: call _get_TTBR0_Type_ASID(s_14_1)
        let s_14_2: u8 = u_get_TTBR0_Type_ASID(state, tracer, s_14_1);
        // D s_14_3: write-var ga#21505 <= s_14_2
        fn_state.ga_21505 = s_14_2;
        // N s_14_4: jump b13
        return block_13(state, tracer, fn_state);
    }
}
